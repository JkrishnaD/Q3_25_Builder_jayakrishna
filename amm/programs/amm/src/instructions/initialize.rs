use anchor_lang::prelude::*;
use anchor_spl::{ associated_token::AssociatedToken, token::{ Mint, Token, TokenAccount } };

use crate::state::Config;

// basically the whole amm contract contains mints, vaults for mints, liquidity pool for the mints
// and finally config account which contains the all information about the pool like authority, fees, mints, etc
#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>, // person who initialize the pool

    pub mint_x: Account<'info, Mint>, // mint of the token in x-axis
    pub mint_y: Account<'info, Mint>, // mint of the token in y-axis

    #[account(
        init,
        payer = initializer,
        seeds = [b"lp", config.key().as_ref()],
        bump,
        mint::authority = config, // authority of the mint is with config account other than that no one can mint or burn
        mint::decimals = 6 // decimals of the liquidity pool token it can be 6 or 9
    )]
    pub mint_lp: Account<'info, Mint>, // mint of the liquidity pool token where both tokens are deposited

    #[account(
        init,
        payer = initializer,
        seeds = [b"conig", seed.to_le_bytes().as_ref()],
        bump,
        space = 8 + Config::INIT_SPACE
    )]
    pub config: Account<'info, Config>, // config account for the pool containing the authority, mints, fees, data

    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint_x, // it stores the x-axis token
        associated_token::authority = config
    )]
    pub vault_x: Account<'info, TokenAccount>, // token account for the x-axis token
    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint_y, // it stores the y-axis token
        associated_token::authority = config
    )]
    pub vault_y: Account<'info, TokenAccount>, // token account for the y-axis token

    // programs used for the transaction
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Initialize<'info> {
    pub fn init(
        &mut self,
        seed: u64, // a seed to make the config account unique
        fee: u16,
        authority: Option<Pubkey>,
        bumps: InitializeBumps // bump seeds for the accounts
    ) -> Result<()> {
        // we use set_inner here because we want to set the data on a single instance of the Config account
        // so that we'll not miss any data when we use it later
        self.config.set_inner(Config {
            seed,
            authority,
            mint_x: self.mint_x.key(),
            mint_y: self.mint_y.key(),
            fee,
            locked: false,
            config_bump: bumps.config,
            lp_bump: bumps.mint_lp,
        });
        Ok(())
    }
}
