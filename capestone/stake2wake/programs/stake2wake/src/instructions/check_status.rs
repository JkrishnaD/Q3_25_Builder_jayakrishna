use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, Token, TokenAccount, TransferChecked},
    token_interface::Mint,
};

use crate::error::Stake2WakeError;
use crate::state::{ChallengeAccount, Treasury};

#[derive(Accounts)]
pub struct CheckStatus<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // user who started the challenge

    #[account(
        mut,
        seeds = [b"challenge", user_challenge.user.key().as_ref(), user_challenge.start_time.to_le_bytes().as_ref()], // using clock here to allow multiple challenges
        bump = user_challenge.bump,
        has_one = user @ Stake2WakeError::Unauthorized // checks who is checking the challenge
    )]
    pub user_challenge: Account<'info, ChallengeAccount>, // user challenge account

    // the associated token account which is related to the user from which the stake will be taken
    #[account(
        mut,
        associated_token::mint = bonk_mint,
        associated_token::authority = user // authority is the user who is starting the challenge
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    pub bonk_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = bonk_mint,
        associated_token::authority = user_challenge // authority is the user who is checking the challenge
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"treasury",treasury.authority.key().as_ref()],
        bump = treasury.bump,
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(
        mut,
        associated_token::mint = bonk_mint,
        associated_token::authority = treasury
    )]
    pub treasury_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub clock: Sysvar<'info, Clock>,
}

impl<'info> CheckStatus<'info> {
    pub fn check_status(&mut self) -> Result<bool> {
        let now = self.clock.unix_timestamp as u64; // it get's the current time
        let challenge = &mut self.user_challenge;

        require!(challenge.is_active, Stake2WakeError::InactiveChallenge); // checks if the challenge is active or not

        challenge.last_check_time = 0;
        let current_day = now.saturating_sub(challenge.start_time) / 86400; // gets the current day since the challenge started

        let first_check_in = challenge.last_check_time == 0;

        if !first_check_in {
            let last_checked_day = challenge
                .last_check_time
                .saturating_sub(challenge.start_time)
                / 86400;

            require!(
                current_day > last_checked_day,
                Stake2WakeError::AlreadyCheckedInToday
            );
        }

        let seconds_from_midnight = now % 86400; // days starts from midnight so it gets the time from midnight in seconds
        let extra_time = 5 * 60; // extra time given to the user to check in

        let min_wake = seconds_from_midnight.saturating_sub(extra_time); // extra time before given wakeup time
        let max_wake = challenge.wakeup_time + extra_time; // extra time after the wakeup time

        require!(
            seconds_from_midnight >= min_wake && seconds_from_midnight <= max_wake,
            Stake2WakeError::MissedWakeupTime
        ); // checks if the user checked in within the allowed time

        if seconds_from_midnight >= min_wake && seconds_from_midnight <= max_wake {
            challenge.completed_days += 1; // add one day to completed days as the user checked in
            challenge.last_check_time = now; // change the checked in time with latest one

            if challenge.completed_days >= challenge.total_days {
                challenge.is_active = false;
            } // if the given completed time is done the the is done so is active will go false

            // so here the user completed the challenge successfully so we need to transfer the stake back to the user
            let cpi_program = self.token_program.to_account_info();

            let cpi_accounts = TransferChecked {
                from: self.vault.to_account_info(),
                to: self.user_token_account.to_account_info(),
                authority: challenge.to_account_info(),
                mint: self.bonk_mint.to_account_info(),
            };

            let seeds: &[&[u8]] = &[
                b"challenge",
                challenge.user.as_ref(),
                &challenge.start_time.to_le_bytes(),
                &[challenge.bump],
            ];
            let signer_seeds: &[&[&[u8]]] = &[&seeds];

            let ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

            transfer_checked(ctx, challenge.stake_amount, 6)?;
            Ok(true)
        } else {
            // as the user failed to complete the challenge we are moving the funds from the vault to the treasury

            let cpi_program = self.token_program.to_account_info();

            let cpi_accounts = TransferChecked {
                from: self.vault.to_account_info(),
                to: self.treasury_ata.to_account_info(),
                authority: challenge.to_account_info(),
                mint: self.bonk_mint.to_account_info(),
            };

            let seeds: &[&[u8]] = &[
                b"challenge",
                challenge.user.as_ref(),
                &challenge.start_time.to_le_bytes(),
                &[challenge.bump],
            ];

            let signer_seeds: &[&[&[u8]]] = &[&seeds];

            let ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

            transfer_checked(ctx, challenge.stake_amount, 6)?;
            Ok(false)
        }
    }
}
