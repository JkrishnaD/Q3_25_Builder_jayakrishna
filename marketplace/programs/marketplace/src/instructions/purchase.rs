use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::transfer_checked,
    token_interface::{
        close_account, CloseAccount, Mint, TokenAccount, TokenInterface, TransferChecked,
    },
};

use crate::{Listing, Marketplace};

#[derive(Accounts)]
#[instruction(name:String)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>, // the one who buys the listing

    #[account(
        seeds = [b"marketplace",name.as_str().as_bytes()],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>, // buyer from where he buys the listing

    // both are the seller related accounts
    #[account(mut)]
    pub seller: SystemAccount<'info>,
    pub seller_mint: InterfaceAccount<'info, Mint>,

    // account which holds the details about the listing
    #[account(
        seeds = [marketplace.key().as_ref(), seller_mint.key().as_ref()],
        bump = listing.bump,
    )]
    pub listing: Account<'info, Listing>,

    // accounts which holds the
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = seller_mint,
        associated_token::authority = buyer
    )]
    pub buyer_ata: InterfaceAccount<'info, TokenAccount>,

    // where the fees goes
    #[account(
        mut,
        seeds = [b"treasury",marketplace.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>,

    // account which holds the nft
    #[account(
        mut,
        associated_token::mint = seller_mint,
        associated_token::authority = listing
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Purchase<'info> {
    // this function helps to send the fees to the treasury account
    // and price amount to the seller account
    pub fn transfer_amounts(&mut self) -> Result<()> {
        let fees = (self.marketplace.fee as u64)
            .checked_mul(self.listing.price)
            .unwrap()
            .checked_div(1000_u64)
            .unwrap(); // this is fee * price/1000

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.buyer.to_account_info(),
            to: self.seller.to_account_info(),
        };

        // here cutting the platform fee and sending the balance to the seller
        let amount = self.listing.price.checked_sub(fees).unwrap();

        let ctx = CpiContext::new(cpi_program, cpi_accounts);

        // transfering the amount bt cutting the fees to the seller
        transfer(ctx, amount)?;

        let cpi_accounts = Transfer {
            from: self.buyer.to_account_info(),
            to: self.treasury.to_account_info(),
        };

        let ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        // transfering the fees to the treasury by the buyer
        transfer(ctx, fees)
    }

    // this function helps to transfer the nft from vault to the buyer ata
    pub fn transfer_nft(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.buyer_ata.to_account_info(),
            authority: self.listing.to_account_info(),
            mint: self.seller_mint.to_account_info(),
        };

        let seeds = &[
            &self.marketplace.key().to_bytes()[..],
            &self.seller_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(ctx, 1, 0)
    }

    // this function closes the vault account which has
    // nothing left after the nft transfer
    pub fn close_vault(&mut self) -> Result<()> {
        let seeds = &[
            &self.marketplace.key().to_bytes()[..],
            &self.seller_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            authority: self.listing.to_account_info(),
            destination: self.seller.to_account_info(),
        };

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        close_account(ctx)
    }
}
