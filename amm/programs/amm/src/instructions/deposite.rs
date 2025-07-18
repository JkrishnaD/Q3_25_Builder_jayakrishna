use std::u64;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ mint_to, transfer, Mint, MintTo, Token, TokenAccount, Transfer },
};
use constant_product_curve::ConstantProduct;
// use constant_product_curve::ConstantProduct;

use crate::{ error::AmmError, state::Config };

#[derive(Accounts)]
pub struct Deposite<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"lp", config.key().as_ref()],
        bump = config.lp_bump
    )]
    pub mint_lp: Account<'info, Mint>,

    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = config
    )]
    pub vault_x: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = config
    )]
    pub vault_y: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = user
    )]
    pub user_x: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = user
    )]
    pub user_y: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_lp,
        associated_token::authority = user
    )]
    pub user_lp: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Deposite<'info> {
    // this function is used to deposite the tokens into the amm pool
    pub fn deposit(&mut self, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        // check if the pool is locked, if it is then we can't deposite
        require!(self.config.locked == false, AmmError::PoolLocked);
        // check if the user has enough tokens to deposite
        require!(amount != 0, AmmError::InvalidAmount);

        // getting the x and y amount to deposit in the pool from the constant product curve
        let (x, y) = match
            self.mint_lp.supply == 0 &&
            self.vault_x.amount == 0 &&
            self.vault_x.amount == 0
        {
            true => (max_x, max_y),
            false => {
                let amount = ConstantProduct::xy_deposit_amounts_from_l(
                    self.vault_x.amount,
                    self.vault_y.amount,
                    self.mint_lp.supply,
                    amount,
                    6
                ).unwrap();
                (amount.x, amount.y)
            }
        };

        // slipage:- it is the difference between the expected amount and the actual amount
        // checking if the x and y amounts are less than or equal to the max_x and max_y
        require!(x <= max_x && y <= max_y, AmmError::SlippageExceeded);

        self.deposite_tokens(true, x)?;
        self.deposite_tokens(false, y)?;

        self.mint_lp_tokens(amount)?;
        Ok(())
    }
    // this fuction helps to deposite the token into the amm pool
    pub fn deposite_tokens(&self, is_x: bool, amount: u64) -> Result<()> {
        let (from, to) = match is_x {
            true => (self.user_x.to_account_info(), self.user_y.to_account_info()),
            false => (self.user_y.to_account_info(), self.user_x.to_account_info()),
        };

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer {
            from,
            to,
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)
    }

    // after the deposit, we need to mint the lp tokens and deposit them into the user's account
    pub fn mint_lp_tokens(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = MintTo {
            mint: self.mint_lp.to_account_info(),
            to: self.user_lp.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let seeds = &[&b"config"[..], &self.config.seed.to_le_bytes(), &[self.config.config_bump]];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(ctx, amount)
    }
}
