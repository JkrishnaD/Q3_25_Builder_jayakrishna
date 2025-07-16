use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{thaw_account, transfer, Mint, ThawAccount, Token, TokenAccount, Transfer},
};

use crate::{error::ErrorCode, StakeAccount, StakeConfig, UserAccount};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user",user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user
    )]
    pub user_nft_ata: Account<'info, TokenAccount>,

    pub nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, StakeConfig>,

    #[account(
        mut,
        seeds = [b"stake", user.key().as_ref(), nft_mint.key().as_ref()],
        bump = stake_account.bump
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(
        mut,
        seeds = [b"vault", nft_mint.key().as_ref()],
        bump,
        token::mint = nft_mint,
        token::authority = config
    )]
    pub vault_pda: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Unstake<'info> {
    pub fn unstake(&mut self) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;

        // checking if the freezing period is over or not
        require!(
            now - self.stake_account.stake_at >= (self.config.freeze_period as i64),
            ErrorCode::NotFrozen
        );

        // checking is the user has any stakes
        require!(self.user_account.amount_staked > 0, ErrorCode::NoStakes);

        // calculating the points for staing period
        let duration = now - self.stake_account.stake_at;
        self.user_account.points += (duration as u32) * (self.config.points_per_stake as u32);

        // from total stakes reducing one for this unstake
        self.user_account.amount_staked -= 1;

        // checking the user is owner of the staked account
        require_keys_eq!(
            self.stake_account.owner,
            self.user.key(),
            ErrorCode::InvalidStakeOwner
        );

        // accounts to unfreeze the nft
        let thaw_accounts = ThawAccount {
            account: self.vault_pda.to_account_info(),
            authority: self.config.to_account_info(),
            mint: self.nft_mint.to_account_info(),
        };

        let thaw_ctx = CpiContext::new(self.token_program.to_account_info(), thaw_accounts);

        thaw_account(thaw_ctx)?;

        // transfering the nft from the vault pda to the user account ata
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault_pda.to_account_info(),
            to: self.user_nft_ata.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let seeds: &[&[u8]; 2] = &[b"config", &[self.config.bump]];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(ctx, 1)?;
        Ok(())
    }
}
