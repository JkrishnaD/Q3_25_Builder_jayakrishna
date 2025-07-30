#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;

pub mod error;
pub mod events;
pub mod instructions;
pub mod state;

use events::*;
use instructions::*;

declare_id!("9E5nutqKTvWYDBWWNnH9gGyJLUQLjKaeABosszTPHhnZ");

#[program]
pub mod stake2wake {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize_treasury(&ctx.bumps)?;

        emit!(InitializeEvent {
            authority: ctx.accounts.authority.key(),
            treasury: ctx.accounts.treasury.key(),
            bonk_mint: ctx.accounts.bonk_mint.key(),
            treasury_ata: ctx.accounts.treasury_ata.key(),
            total_collected: 0,
        });
        Ok(())
    }

    pub fn start_challenge(
        ctx: Context<StartChallenge>,
        start_time: u64,
        wakeup_time: u64,
        stake_amount: u64,
        total_days: u64,
    ) -> Result<()> {
        ctx.accounts.start_challenge(
            start_time,
            wakeup_time,
            stake_amount,
            total_days,
            &ctx.bumps,
        )?;

        emit!(StartChallengeEvent {
            user: ctx.accounts.user.key(),
            user_challenge: ctx.accounts.user_challenge.key(),
            user_token_account: ctx.accounts.user_token_account.key(),
            bonk_mint: ctx.accounts.bonk_mint.key(),
            vault: ctx.accounts.vault.key(),
            wakeup_time,
            stake_amount,
            total_days,
        });
        Ok(())
    }
    pub fn check_status(ctx: Context<CheckStatus>) -> Result<()> {
        let did_complete = ctx.accounts.check_status()?; // getting the status of the challenge
        let challenge = &ctx.accounts.user_challenge;

        // bool to find the wether user recieves the bonk or not
        let tokens_returned = did_complete && challenge.completed_days == challenge.total_days;
        let was_failed = !challenge.is_active && !tokens_returned;

        emit!(CheckStatusEvent {
            user: ctx.accounts.user.key(),
            user_challenge: challenge.key(),
            completed_days: challenge.completed_days,
            last_check_time: challenge.last_check_time,
            is_active: challenge.is_active,
            did_complete,
            tokens_returned,
            was_failed,
        });

        Ok(())
    }

    pub fn cancel_challenge(ctx: Context<CancelChallenge>) -> Result<()> {
        // Call the internal logic
        ctx.accounts.cancel_challenge()?;

        // Extract necessary fields for event
        let challenge = &ctx.accounts.user_challenge;
        let treasury_ata = ctx.accounts.treasury_ata.key();
        let vault = ctx.accounts.vault.key();
        let timestamp = ctx.accounts.clock.unix_timestamp as u64;

        let stake_amount = challenge.stake_amount;
        let user_balance = ctx.accounts.user_token_account.amount;

        let return_amount = user_balance;
        let penalty_amount = stake_amount.saturating_sub(return_amount);

        // Emit event
        emit!(CancelChallengeEvent {
            user: ctx.accounts.user.key(),
            user_challenge: challenge.key(),
            penalty_amount,
            return_amount,
            vault,
            treasury_ata,
            timestamp,
        });

        Ok(())
    }

    pub fn treasury_withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;

        emit!(WithdrawEvent {
            amount,
            authority: ctx.accounts.authority.key(),
            authority_ata: ctx.accounts.authority_ata.key(),
            bonk_mint: ctx.accounts.bonk_mint.key(),
            treasury: ctx.accounts.treasury.key(),
            treasury_ata: ctx.accounts.treasury_ata.key(),
        });
        Ok(())
    }
}
