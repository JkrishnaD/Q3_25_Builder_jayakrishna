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

    pub fn initialize(ctx: Context<Make>,seed:u64,recieve:u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, recieve, &ctx.bumps)?;
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn maker_deposite(ctx: Context<Make>,amount:u64)->Result<()>{
        ctx.accounts.deposit(amount)?;
        msg!("Maker deposited {} tokens", amount);
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>)->Result<()>{
        ctx.accounts.refund_and_close()?;
        msg!("Refunded and closed escrow account");
        Ok(())
    }

    pub fn taker_deposite(ctx:Context<Take>,amount:u64)->Result<()>{
        ctx.accounts.deposite(amount)?;
        msg!("Taker deposited {} tokens", amount);
        Ok(())
    }

    pub fn taker_take_and_close(ctx:Context<Take>)->Result<()>{
        ctx.accounts.take_and_close()?;
        msg!("Taker took and closed escrow account");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}