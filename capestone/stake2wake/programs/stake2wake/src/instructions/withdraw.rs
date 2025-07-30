use std::str::FromStr;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ transfer_checked, Mint, Token, TokenAccount, TransferChecked },
};

use crate::{ error::Stake2WakeError, state::Treasury };

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = bonk_mint,
        associated_token::authority = authority
    )]
    pub authority_ata: Account<'info, TokenAccount>, // authority token account

    #[account(
        mut,
        seeds = [b"treasury",authority.key().as_ref()],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(
        mut,
        associated_token::mint = bonk_mint,
        associated_token::authority = treasury
    )]
    pub treasury_ata: Account<'info, TokenAccount>,

    #[account(mint::token_program = token_program)]
    pub bonk_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let admin_pubkey = Pubkey::from_str(
            "Bt9AAsmv7ocm2kJsusYrk2gG1Sm6Fy6rS6dRtiC8xFGX"
        ).unwrap();

        require_eq!(*self.authority.key, admin_pubkey, Stake2WakeError::Unauthorized);
        require!(self.treasury.total_collected >= amount, Stake2WakeError::InsufficientFunds);

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.treasury_ata.to_account_info(),
            to: self.authority_ata.to_account_info(),
            mint: self.bonk_mint.to_account_info(),
            authority: self.treasury.to_account_info(),
        };

        let authority_key = self.authority.key();

        let seeds: &[&[u8]; 3] = &[b"treasury", authority_key.as_ref(), &[self.treasury.bump]];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        let decimals = self.bonk_mint.decimals;

        transfer_checked(ctx, amount, decimals)?;

        self.treasury.total_collected = self.treasury.total_collected.saturating_sub(amount);
        Ok(())
    }
}
