use anchor_lang::prelude::*;
use anchor_spl::{token::Token, token_interface::Mint};

use crate::Marketplace;

#[derive(Accounts)]
#[instruction(name:String)]
pub struct Initialize<'info> {
    // the admin is the person who creates the marketplace
    #[account(mut)]
    pub admin: Signer<'info>,

    // main account which holds the every information about the marketplace
    // it is initialized by the admin and holds the admin's public key, fee, and
    // the bump seeds for the marketplace, treasury, and reward mint.
    // the name is used to identify the marketplace and is used as a seed for the
    #[account(
        init,
        payer = admin,
        seeds = [b"marketplace",name.as_str().as_bytes()],
        bump,
        space = 8+Marketplace::INIT_SPACE
    )]
    pub marketplace: Account<'info, Marketplace>,

    // The treasury is a system account that holds the fees collected from the marketplace
    #[account(
        seeds = [b"marketplace",marketplace.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>, //  as this is a system account so no need to initiate this manually

    #[account(
        init,
        payer = admin,
        seeds = [b"reward",marketplace.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = admin
    )]
    pub reward_mint: InterfaceAccount<'info, Mint>, // reward mint is created by the admin

    // programs accounts
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, name: String, fee: u16, bumps: &InitializeBumps) -> Result<()> {
        // an instance of the Marketplace struct is created and initialized with the provided parameters
        self.marketplace.set_inner(Marketplace {
            admin: self.admin.key(),
            fee,
            bump: bumps.marketplace,
            treasury_bump: bumps.treasury,
            reward_bump: bumps.reward_mint,
            name,
        });
        Ok(())
    }
}
