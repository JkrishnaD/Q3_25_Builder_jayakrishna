#![allow(unexpected_cfgs)]
#![allow(deprecated)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("8vFkpy4ZNCVHGCMTMoetzwGwcwaq44xfW9fgAGkjF1dH");

#[program]
pub mod marketplace {
    use super::*;

    pub fn init_marketplace(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)
    }

    pub fn listing(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.list(price, &ctx.bumps)?;
        ctx.accounts.deposite_nft()
    }

    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.transfer_amounts()?;
        ctx.accounts.transfer_nft()?;
        ctx.accounts.close_vault()
    }

    pub fn delisting(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.withdraw_nft()?;
        ctx.accounts.close_account()
    }
}
