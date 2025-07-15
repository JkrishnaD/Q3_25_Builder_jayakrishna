use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ transfer, Mint, Token, TokenAccount, Transfer },
};

use crate::{ StakeAccount, StakeConfig, UserAccount };

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // user who is staking the NFT

    // user accounts which stores the user stake details
    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    // config account which stores the staking configurations
    #[account(
        mut,
        seeds = [b"config"],
        bump=config.bump
    )]
    pub config: Account<'info, StakeConfig>,

    // the NFT mint which is being staked
    pub nft_mint: Account<'info, Mint>,

    // the user token account from where the nft is transfered to vault
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user
    )]
    pub user_nft_ata: Account<'info, TokenAccount>,

    // the stake account where the nft is staked
    #[account(
        init,
        payer = user,
        space = 8 + StakeAccount::INIT_SPACE,
        seeds = [b"stake", user.key().as_ref(), nft_mint.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,

    // the vault pda which holds the staked nft
    #[account(
        init_if_needed,
        payer = user,
        seeds = [b"vault", nft_mint.key().as_ref()],
        bump,
        token::mint = nft_mint,
        token::authority = vault_authority
    )]
    pub vault_pda: Account<'info, TokenAccount>,

    // the vault authority which is a PDA that controls the vault
    #[account(seeds = [b"vault_authority"], bump)]
    pub vault_authority: UncheckedAccount<'info>,

    // program accounts used for token operations
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self, bumps: &StakeBumps) -> Result<()> {
        let clock = Clock::get()?;

        // updating the stake config
        self.stake_account.set_inner(StakeAccount {
            owner: self.user.key(),
            mint: self.nft_mint.key(),
            stake_at: clock.unix_timestamp,
            bump: bumps.stake_account,
        });

        // updating the user account details like points and amount staked
        self.user_account.points += self.config.points_per_stake as u32;
        self.user_account.amount_staked += 1;

        // transfering the nft from user ata to vault pda
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.user_nft_ata.to_account_info(),
            to: self.vault_pda.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(ctx, 1)?;
        Ok(())
    }
}
