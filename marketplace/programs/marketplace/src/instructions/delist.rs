use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::close_account,
    token_interface::{
        transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface, TransferChecked,
    },
};

use crate::{error::MarketplaceErrors, Listing, Marketplace};

#[derive(Accounts)]
pub struct Delist<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    // account which stores the listing details
    #[account(
        mut,
        close = seller, // as we close the account any balance in that to seller
        seeds = [marketplace.key().as_ref(), seller_mint.key().as_ref()],
        has_one = seller,
        bump = listing.bump
    )]
    pub listing: Account<'info, Listing>,

    // nft mint which is kept for sale in listing
    pub seller_mint: InterfaceAccount<'info, Mint>,

    // account which is storing the nft
    #[account(
        associated_token::mint = seller_mint,
        associated_token::authority = seller
    )]
    pub seller_ata: InterfaceAccount<'info, TokenAccount>,

    // account whcih has the marketplace details
    #[account(
        seeds = [b"marketplace",marketplace.name.as_str().as_bytes()],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    // account where the nft is kept in hold
    #[account(
        mut,
        associated_token::mint = seller_mint,
        associated_token::authority = seller
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Delist<'info> {
    pub fn withdraw_nft(&mut self) -> Result<()> {
        require!(
            self.seller.key() == self.listing.seller.key(),
            MarketplaceErrors::Unauthorized,
        );

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.seller_ata.to_account_info(),
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

    pub fn close_account(&mut self) -> Result<()> {
        //
        require!(
            self.seller.key() == self.listing.seller.key(),
            MarketplaceErrors::Unauthorized,
        );

        // close the listing account and transfer the lamports to the seller
        // the listing account is closed by the seller
        // the lamports are transferred to the seller

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
