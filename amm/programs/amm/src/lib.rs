#![allow(unexpected_cfgs)]
#![allow(deprecated)]

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod events;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;
pub use events::*;

declare_id!("4Sjvuze4bNMnX4fJYGAgrHnjVe8z98hNd94HVGTPq8qA");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>
    ) -> Result<()> {
        ctx.accounts.init(seed, fee, authority, ctx.bumps)?;

        emit!(InitializeEvent {
            initializer: ctx.accounts.initializer.key(),
            config: ctx.accounts.config.key(),
            mint_lp: ctx.accounts.mint_lp.key(),
            mint_x: ctx.accounts.mint_x.key(),
            mint_y: ctx.accounts.mint_y.key(),
            vault_x: ctx.accounts.vault_x.key(),
            vault_y: ctx.accounts.vault_y.key(),
            fee,
        });
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposite>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y)?;

        emit!(DepositEvent {
            user: ctx.accounts.user.key(),
            mint_x: ctx.accounts.mint_x.key(),
            mint_y: ctx.accounts.mint_y.key(),
            mint_lp: ctx.accounts.mint_lp.key(),
            vault_x: ctx.accounts.vault_x.key(),
            vault_y: ctx.accounts.vault_y.key(),
            user_lp: ctx.accounts.user_lp.key(),
            config: ctx.accounts.config.key(),
            max_x,
            max_y,
            amount,
        });
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64, min_x: u64, min_y: u64) -> Result<()> {
        ctx.accounts.withdraw(amount, min_x, min_y)?;

        emit!(WithdrawEvent {
            user: ctx.accounts.user.key(),
            mint_x: ctx.accounts.mint_x.key(),
            mint_y: ctx.accounts.mint_y.key(),
            mint_lp: ctx.accounts.mint_lp.key(),
            vault_x: ctx.accounts.vault_x.key(),
            vault_y: ctx.accounts.vault_y.key(),
            user_lp: ctx.accounts.user_lp.key(),
            config: ctx.accounts.config.key(),
            amount,
            min_x,
            min_y,
        });
        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, amount: u64, is_x: bool, min: u64) -> Result<()> {
        ctx.accounts.swap(amount, is_x, min)?;

        emit!(SwapEvent {
            user: ctx.accounts.user.key(),
            mint_x: ctx.accounts.mint_x.key(),
            mint_y: ctx.accounts.mint_y.key(),
            mint_lp: ctx.accounts.mint_lp.key(),
            vault_x: ctx.accounts.vault_x.key(),
            vault_y: ctx.accounts.vault_y.key(),
            config: ctx.accounts.config.key(),
            amount,
            min,
        });
        Ok(())
    }

    pub fn lock(ctx: Context<Update>) -> Result<()> {
        ctx.accounts.lock()?;

        emit!(UpdateEvent {
            user: ctx.accounts.user.key(),
            config: ctx.accounts.config.key(),
            locked: true,
        });
        Ok(())
    }

    pub fn unlock(ctx: Context<Update>) -> Result<()> {
        ctx.accounts.unlock()?;

        emit!(UpdateEvent {
            user: ctx.accounts.user.key(),
            config: ctx.accounts.config.key(),
            locked: false,
        });
        Ok(())
    }
}
