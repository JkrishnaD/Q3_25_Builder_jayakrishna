use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ transfer_checked, Token, TokenAccount, TransferChecked },
    token_interface::Mint,
};

use crate::state::{ ChallengeAccount, Treasury };
use crate::error::Stake2WakeError;

#[derive(Accounts)]
pub struct CancelChallenge<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // usert should sign
    #[account(
        mut,
        seeds = [b"challenge", user_challenge.user.key().as_ref(), user_challenge.start_time.to_le_bytes().as_ref()], // using clock here to allow multiple challenges
        bump = user_challenge.bump,
        has_one = user @ Stake2WakeError::Unauthorized,
        close = user
    )]
    pub user_challenge: Account<'info, ChallengeAccount>,
    // challenge account

    #[account(
        mut,
        associated_token::mint = bonk_mint,
        associated_token::authority = user
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    //users token ata (user wallet)

    pub bonk_mint: InterfaceAccount<'info, Mint>,
    // bon tokens mint

    #[account(
        mut,
        associated_token::mint = bonk_mint,
        associated_token::authority = user_challenge
    )]
    pub vault: Account<'info, TokenAccount>,
    // vault account where users staked amount will be stored

    #[account(
        mut,
        seeds = [b"treasury", treasury.authority.key().as_ref()],
        bump = treasury.bump,
    )]
    pub treasury: Account<'info, Treasury>,
    // treasury account where the penalties and fees will be stored

    #[account(
        mut,
        associated_token::mint = bonk_mint,
        associated_token::authority = treasury
    )]
    pub treasury_ata: Account<'info, TokenAccount>,
    // treasury's bonk token address

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,

    pub clock: Sysvar<'info, Clock>,
}

impl<'info> CancelChallenge<'info> {
    pub fn cancel_challenge(&mut self) -> Result<()> {
        let challenge = &mut self.user_challenge;
        require!(challenge.is_active, Stake2WakeError::InactiveChallenge);
        // checking whether the challenge is active or not

        let now = self.clock.unix_timestamp as u64;
        let total_duration = challenge.end_time.saturating_sub(challenge.start_time);
        let elapsed_time = now.saturating_sub(challenge.start_time);
        //declaration

        // Calculate progress as a percentage (0 to 100) for linear interpolation calculation
        let progress = if total_duration > 0 { (elapsed_time * 100) / total_duration } else { 0 };

        // Dynamic penalty: 20% early (0-25%), linear decrease to 5% late (75-100%)
        let penalty_percentage = if progress <= 25 {
            20 // Early cancellation: 20% penalty
        } else if progress >= 75 {
            5 // Late cancellation: 5% penalty
        } else {
            // Linear interpolation between 20% and 5% for 25% to 75% progress
            20 - ((progress - 25) * (20 - 5)) / (75 - 25)
        };

        let penalty_amount = (challenge.stake_amount * penalty_percentage) / 100;
        // stake amount * percent in decimals to calculate the penalty
        let return_amount = challenge.stake_amount.saturating_sub(penalty_amount);
        // stake - penalty = return amount

        // Update challenge status
        challenge.is_active = false;

        // Update treasury total collected , total = previos total + recently recieved
        self.treasury.total_collected =
            self.treasury.total_collected.saturating_add(penalty_amount);

        // Transfer penalty to treasury by performaing a cpi
        if penalty_amount > 0 {
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
            transfer_checked(ctx, penalty_amount, 6)?;
        }

        // Transfer remaining amount back to user
        if return_amount > 0 {
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
            transfer_checked(ctx, return_amount, 6)?;
        }

        Ok(())
    }
}
