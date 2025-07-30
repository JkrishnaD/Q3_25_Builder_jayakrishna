use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, Token, TokenAccount, TransferChecked},
    token_interface::Mint,
};

use crate::error::Stake2WakeError;
use crate::state::ChallengeAccount;

#[derive(Accounts)]
#[instruction(start_time: u64)]
pub struct StartChallenge<'info> {
    // the person who is starting the challenge
    #[account(mut)]
    pub user: Signer<'info>,

    // Creates a new PDA account to store user challenge data
    #[account(
        init,
        payer = user,
        space = 8 + ChallengeAccount::INIT_SPACE, // 8 bytes for discriminator + struct size
        seeds = [b"challenge", user.key().as_ref(), start_time.to_le_bytes().as_ref()], // using clock here to allow multiple challenges
        bump
    )]
    pub user_challenge: Account<'info, ChallengeAccount>, // we are creating account for userchallenge first

    // the associated token account which is related to the user from which the stake will be taken
    #[account(
        mut,
        associated_token::mint = bonk_mint,
        associated_token::authority = user // authority is the user who is starting the challenge
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    // The mint of the BONK token which is being staked
    pub bonk_mint: InterfaceAccount<'info, Mint>,

    // The vault where the stake will be stored (
    #[account(
        init,
        payer = user, // payer is the user who is starting the challenge
        associated_token::mint = bonk_mint,
        associated_token::authority = user_challenge // authority is the challenge account
    )]
    pub vault: Account<'info, TokenAccount>,

    // SPL Token program for performing token transfer
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    // Clock sysvar to get current onchain timestamp
    pub clock: Sysvar<'info, Clock>,
}

impl<'info> StartChallenge<'info> {
    pub fn start_challenge(
        &mut self,
        start_time: u64,
        wakeup_time: u64,  // When the user wants to wake up
        stake_amount: u64, // Amount user is staking
        total_days: u64,   // Number of days challenge lasts
        bumps: &StartChallengeBumps,
    ) -> Result<()> {
        require!(stake_amount > 0, Stake2WakeError::InvalidStakeAmount); // checking for the valid stake amount
        require!(total_days > 0, Stake2WakeError::InvalidTotalDays); // checking for the valid total days
        require!(
            wakeup_time > (self.clock.unix_timestamp as u64),
            Stake2WakeError::InvalidWakeupTime
        ); // checking for the valid wakeup time

        // creating an instance using set_inner method to update all the fileds at once
        self.user_challenge.set_inner(ChallengeAccount {
            user: self.user.key(),
            wakeup_time,
            stake_amount,
            vault_ata: self.vault.key(),
            mint: self.bonk_mint.key(),
            vault: self.vault.key(),
            is_active: true,
            start_time,
            end_time: start_time + total_days * 86400, // 86400 seconds in a day
            last_check_time: 0,                        // Initialize last check time to 0
            completed_days: 0,
            total_days,
            bump: bumps.user_challenge,
        });

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.user_token_account.to_account_info(),
            mint: self.bonk_mint.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, stake_amount, 6)
    }
}
