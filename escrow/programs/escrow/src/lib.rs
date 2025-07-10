#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;

declare_id!("CwgCa5b6vwB5DLRdtAc8rQ9gFhqUvBEoT5mk1Uh3zLWd");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Make>,seed:u64,recieve:u64,deposit_amt:u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, recieve, &ctx.bumps)?;
        ctx.accounts.deposit(deposit_amt)?;

        Ok(())
    }

    pub fn refund(ctx: Context<Refund>)->Result<()>{
        msg!("RefundEscrow instruction called");
        ctx.accounts.refund_and_close()?;
        Ok(())
    }

    pub fn take(ctx:Context<Take>,amount:u64)->Result<()>{
        ctx.accounts.transfer_to_maker(amount)?;
        ctx.accounts.take_and_close()?;
        Ok(())
    }
}