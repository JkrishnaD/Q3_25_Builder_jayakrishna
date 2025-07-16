#![allow(deprecated)]
#![allow(unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod events;
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
    use crate::events::*;

    pub fn initializ_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.initalize_user(ctx.bumps)?;

        emit!(InitializeUserEvent {
            amount_staked: ctx.accounts.user_account.amount_staked,
            points: ctx.accounts.user_account.points,
            user: ctx.accounts.user.key(),
        });
        Ok(())
    }

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        max_stake: u8,
        points_per_stake: u8,
        freeze_period: u32,
    ) -> Result<()> {
        ctx.accounts
            .initialize_config(max_stake, points_per_stake, freeze_period, &ctx.bumps)?;

        emit!(InitializeConfigEvent {
            config: ctx.accounts.config.key(),
            max_stake: ctx.accounts.config.max_stake,
            points_per_stake: ctx.accounts.config.points_per_stake,
            freeze_period: ctx.accounts.config.freeze_period,
        });
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)?;

        emit!(StakeEvent {
            owner: ctx.accounts.user.key(),
            nft_mint: ctx.accounts.nft_mint.key(),
            stake_at: ctx.accounts.stake_account.stake_at,
        });
        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()?;

        emit!(UnstakeEvent {
            owner: ctx.accounts.user.key(),
            nft_mint: ctx.accounts.nft_mint.key(),
            amount_staked: ctx.accounts.user_account.amount_staked,
            stake_account: ctx.accounts.stake_account.key(),
        });
        Ok(())
    }
}
