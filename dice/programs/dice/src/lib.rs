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

declare_id!("HYJiJVyBohJ18UBQrnB3jMr3Wwxk5wiHN1qP2gSqm89y");

#[program]
pub mod dice {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,amount:u64) -> Result<()> {
        ctx.accounts.init(amount)?;
        Ok(())
    }
}
