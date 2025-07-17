use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        mpl_token_metadata::instructions::{
            ThawDelegatedAccountCpi, ThawDelegatedAccountCpiAccounts,
        },
        Metadata, MetadataAccount,
    },
    token::{revoke, Mint, Revoke, Token, TokenAccount},
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
        bump = stake_account.bump,
        close = user
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

    pub collection_mint: Account<'info, Mint>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            nft_mint.key().as_ref(),
            b"edition"
        ],
        bump,
        seeds::program = metadata_program.key(),
    )]
    pub edition: Account<'info, MetadataAccount>,

    pub metadata_program: Program<'info, Metadata>,
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
        let duration = ((now - self.stake_account.stake_at) / 86400) as u32;
        self.user_account.points += (duration as u32) * (self.config.points_per_stake as u32);

        let seeds = &[
            b"stake",
            self.user.to_account_info().key.as_ref(),
            self.nft_mint.to_account_info().key.as_ref(),
            &[self.stake_account.bump],
        ];

        let signers_seeds = &[&seeds[..]];

        let thaw_accounts = ThawDelegatedAccountCpiAccounts {
            delegate: &self.stake_account.to_account_info(),
            edition: &self.edition.to_account_info(),
            mint: &self.nft_mint.to_account_info(),
            token_account: &self.user_nft_ata.to_account_info(),
            token_program: &self.token_program.to_account_info(),
        };

        ThawDelegatedAccountCpi::new(&self.token_program.to_account_info(), thaw_accounts)
            .invoke_signed(signers_seeds)?;

        // from total stakes reducing one for this unstake
        self.user_account.amount_staked -= 1;

        let revoke_accounts = Revoke {
            authority: self.user.to_account_info(),
            source: self.user_nft_ata.to_account_info(),
        };

        let revoke_ctx = CpiContext::new(self.token_program.to_account_info(), revoke_accounts);

        revoke(revoke_ctx)?;
        Ok(())
    }
}
