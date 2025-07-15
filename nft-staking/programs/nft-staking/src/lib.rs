#![allow(deprecated)]
#![allow(unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("82d2QMPyH2q8Y2LnJhZNuFT6wN8d3t4x7MMXZ8Tde57V");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initializ_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.initalize_user(ctx.bumps)
    }

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        ctx.accounts.initialize_config(
            ctx.accounts.config.max_stake,
            ctx.accounts.config.points_per_stake,
            ctx.accounts.config.freeze_period,
            &ctx.bumps
        )
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake(&ctx.bumps)
    }
}
