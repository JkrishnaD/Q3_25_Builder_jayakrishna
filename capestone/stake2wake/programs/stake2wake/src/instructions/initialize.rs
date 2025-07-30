use std::str::FromStr;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{ Mint, TokenAccount, TokenInterface },
};
use crate::{ error::Stake2WakeError, state::Treasury };

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>, // the authority who initiates the process

    #[account(
        init,
        payer = authority,
        space = 8 + Treasury::INIT_SPACE,
        seeds = [b"treasury", authority.key().as_ref()],
        bump
    )]
    pub treasury: Account<'info, Treasury>, // account which holds the treasury information

    #[account(mint::token_program = token_program)]
    pub bonk_mint: InterfaceAccount<'info, Mint>, // the mint of the BONK token

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = bonk_mint,
        associated_token::authority = treasury
    )]
    pub treasury_ata: InterfaceAccount<'info, TokenAccount>, // associated token account for BONK treasury

    // programs useful for the transaction
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Initialize<'info> {
    pub fn initialize_treasury(&mut self, bumps: &InitializeBumps) -> Result<()> {
        let admin_pubkey = Pubkey::from_str(
            "Bt9AAsmv7ocm2kJsusYrk2gG1Sm6Fy6rS6dRtiC8xFGX"
        ).unwrap();

        // checking wether the user who is intiating the process is the authority
        require_eq!(self.authority.key(), admin_pubkey, Stake2WakeError::Unauthorized);
        self.treasury.set_inner(Treasury {
            authority: self.authority.key(),
            bonk_mint: self.bonk_mint.key(),
            treasury_ata: self.treasury_ata.key(),
            bump: bumps.treasury,
            total_collected: 0,
        });
        Ok(())
    }
}
