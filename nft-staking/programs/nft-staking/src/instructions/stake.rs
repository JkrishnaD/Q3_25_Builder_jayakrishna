use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
    token::{
        approve, freeze_account, transfer, Approve, FreezeAccount, Mint, Token, TokenAccount,
        Transfer,
    },
};

use crate::{error::ErrorCode, StakeAccount, StakeConfig, UserAccount};

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
        token::authority = config
    )]
    pub vault_pda: Account<'info, TokenAccount>,

    #[account(
        seeds=[
            b"metadata",
            metadata_program.key().as_ref(),
            nft_mint.key().as_ref()
        ],bump,
        seeds::program = metadata_program.key(),
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true,
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds=[
            b"metadata",
            metadata_program.key().as_ref(),
            nft_mint.key().as_ref(),
            b"edition"
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    pub edition: Account<'info, MasterEditionAccount>,

    pub collection_mint: Account<'info, Mint>,

    // program accounts used for token operations
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub metadata_program: Program<'info, Metadata>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self, bumps: &StakeBumps) -> Result<()> {
        let clock = Clock::get()?;

        // checkin the user owner-ship here
        require!(self.user_nft_ata.amount == 1, ErrorCode::InvalidNftAmount);

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

        // accounts involved in the approve
        let cpi_approve = Approve {
            to: self.user_nft_ata.to_account_info(),
            authority: self.user.to_account_info(), // the one who owns the mint
            delegate: self.stake_account.to_account_info(), // for whom we are providing the authority temporarily
        };

        // context for approving the nft
        let approve_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_approve);
        approve(approve_ctx, 1)?;

        // transfering the nft from the user to vault after being approved
        let cpi_accounts = Transfer {
            from: self.user_nft_ata.to_account_info(),
            to: self.vault_pda.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let transfer_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        transfer(transfer_ctx, 1)?;

        // after the nft being staked user cannot be using that we are freezing that nft
        let freeze_accounts = FreezeAccount {
            account: self.vault_pda.to_account_info(), // account where we freeze
            authority: self.config.to_account_info(),  // who has the authority over the freeze
            mint: self.nft_mint.to_account_info(),     // which nft is going to be freezed
        };

        let freeze_ctx = CpiContext::new(self.token_program.to_account_info(), freeze_accounts);

        freeze_account(freeze_ctx)?;
        Ok(())
    }
}
