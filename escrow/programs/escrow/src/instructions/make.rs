use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{ transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked },
};

use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Make<'info> {
    // maker who creates the account
    #[account(mut)]
    pub maker: Signer<'info>,

    // tokens which sent by the maker
    #[account(mint::token_program = token_program)]
    pub mint_a: InterfaceAccount<'info, Mint>,

    // token which taker send to maker
    #[account(mint::token_program = token_program)]
    pub mint_b: InterfaceAccount<'info, Mint>,

    // token account of the maker which will recieve token from taker
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    // escrow account which holds the state of the escrow
    #[account(
        init,
        payer = maker,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
        space = 8 + Escrow::INIT_SPACE
    )]
    pub escrow: Account<'info, Escrow>,

    // vault account where the maker deposits their tokens
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    // programs required for the transaction
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Make<'info> {
    // function to initialize the escrow account
    pub fn init_escrow(&mut self, seed: u64, recieve: u64, bumps: &MakeBumps) -> Result<()> {
        // set_inner is used to change the all vaules in single instance
        self.escrow.set_inner(Escrow {
            seed,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            recieve,
            bump: bumps.escrow,
        });
        Ok(())
    }

    // function to deposit the tokens into the escrow vault
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        // it does the verification before the transaction
        let transfer_accounts = TransferChecked {
            from: self.maker_ata_a.to_account_info(),
            authority: self.maker.to_account_info(),
            to: self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);

        // transfer_checked is used to transfer the tokens from maker to vault more securely
        transfer_checked(cpi_ctx, amount, self.mint_a.decimals)?;
        Ok(())
    }
}
