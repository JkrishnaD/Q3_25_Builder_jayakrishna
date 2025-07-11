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

declare_id!("4Sjvuze4bNMnX4fJYGAgrHnjVe8z98hNd94HVGTPq8qA");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fee: u64,
        authority: Option<Pubkey>
    ) -> Result<()> {
        ctx.accounts.init(seed, fee, authority, ctx.bumps)
    }
}
