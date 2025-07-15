use anchor_lang::prelude::*;
use anchor_spl::token::{ Mint, Token };

use crate::StakeConfig;

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(init, payer = admin, seeds = [b"config"], bump, space = 8 + StakeConfig::INIT_SPACE)]
    pub config: Account<'info, StakeConfig>,

    #[account(
        init,
        payer = admin,
        seeds = [b"rewards", config.key().as_ref()],
        bump,
        mint::authority = admin,
        mint::decimals = 6
    )]
    pub reward_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,
}

impl<'info> InitializeConfig<'info> {
    pub fn initialize_config(
        &mut self,
        max_stake: u8,
        points_per_stake: u8,
        freeze_period: u32,
        bumps: &InitializeConfigBumps
    ) -> Result<()> {
        self.config.set_inner(StakeConfig {
            max_stake,
            freeze_period,
            points_per_stake,
            reward_bump: bumps.reward_mint,
            bump: bumps.config,
        });
        Ok(())
    }
}
