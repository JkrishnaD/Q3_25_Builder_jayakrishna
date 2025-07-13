use anchor_lang::prelude::*;

#[account] // this defines as this is the account
#[derive(InitSpace)] // this calculates the space of the account automatically
pub struct Config {
    pub seed: u64, // to find the config account as `[seed, authority, mint_x, mint_y]` because we have multiple configs for different pairs
    pub authority: Option<Pubkey>, // no authority means the config is for a public pool
    pub mint_x: Pubkey, // the mint of the token in the x axis
    pub mint_y: Pubkey, // the mint of the token in the y axis
    pub fee: u16, // percentage fees for the pool, e.g. 0.3% = 3000
    pub locked: bool, // if it is true then no trading is allowed
    pub config_bump: u8, // bump seed to fing this particular
    pub lp_bump: u8, // bump seed to find this liquidity pool token
}

