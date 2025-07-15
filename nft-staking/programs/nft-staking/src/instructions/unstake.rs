use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ transfer, Mint, Token, TokenAccount, Transfer },
};

use crate::{ StakeAccount, StakeConfig, UserAccount, error::ErrorCode };

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
        token::authority = vault_authority
    )]
    pub vault_pda: Account<'info, TokenAccount>,

    #[account(seeds = [b"vault_authority"], bump)]
    pub vault_authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Unstake<'info> {
    pub fn unstake(&mut self, bumps: &UnstakeBumps) -> Result<()> {
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

        // transfering the nft from the vault pda to the user account ata
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault_pda.to_account_info(),
            to: self.user_nft_ata.to_account_info(),
            authority: self.vault_authority.to_account_info(),
        };

        let seeds: &[&[u8]; 2] = &[b"vault_authority", &[bumps.vault_authority]];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(ctx, 1)?;
        Ok(())
    }
}
