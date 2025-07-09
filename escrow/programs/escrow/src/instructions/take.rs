use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        transfer_checked,
        close_account,
        Mint,
        TokenAccount,
        TokenInterface,
        TransferChecked,
        CloseAccount,
    },
};

use crate::state::Escrow;

#[derive(Accounts)]
pub struct Take<'info> {
    // this is the taker who sends the token to the maker
    #[account(mut)]
    pub taker: Signer<'info>,

    //maker who started the escrow and recieves token from taker
    #[account(mut)]
    pub maker: SystemAccount<'info>,

    // these are the token which is sent by the maker
    #[account(mut,mint::token_program = token_program)]
    pub mint_a: InterfaceAccount<'info, Mint>,

    // these are the token the taker will send to maker
    #[account(mut,mint::token_program = token_program)]
    pub mint_b: InterfaceAccount<'info, Mint>,

    // these are the token accounts of the taker receiving the token from maker
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority= taker,
        associated_token::token_program=token_program
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,

    // these are the token accounts of the taker sends token to maker
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,

    // these are the token accounts of the maker receiving the token from taker
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,

    // this is the escrow account which holds the state of the escrow
    #[account(
        mut,
        close = maker, // as maker created while closing rent is sent to the maker
        has_one = maker, // it should has maker
        has_one = mint_a,// it should contain mint_a
        has_one = mint_b,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,

    // this is the account which holds the tokens for the escrow
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    // these are the program accounts
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
    // function to deposite the tokens by taker into the escrow vault
    pub fn deposite(&mut self, amount: u64) -> Result<()> {
        // it includes all the account whic are involved in the deposite transaction
        // transfer checked helps program to check verify the accounts,mint and the decimals
        let transer_accounts = TransferChecked {
            from: self.taker_ata_a.to_account_info(),
            authority: self.taker.to_account_info(),
            mint: self.mint_b.to_account_info(),
            to: self.vault.to_account_info(),
        };

        // adding the program and th accounts into the cpi context
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transer_accounts);

        // making a cpi and transefer the token from taker to vault
        transfer_checked(cpi_ctx, amount, self.mint_a.decimals)?;

        Ok(())
    }
    pub fn take_and_close(&mut self) -> Result<()> {
        let seeds: [&[&[u8]]; 1] = [
            &[
                b"escrow",
                self.maker.to_account_info().key.as_ref(),
                &self.escrow.seed.to_le_bytes()[..],
                &[self.escrow.bump],
            ],
        ];

        let transfer_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
            mint: self.mint_a.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            transfer_accounts,
            &seeds
        );

        transfer_checked(cpi_ctx, self.vault.amount, self.mint_a.decimals)?;

        let close_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            authority: self.escrow.to_account_info(),
            destination: self.maker.to_account_info(),
        };

        let close_cpi = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            close_accounts,
            &seeds
        );

        close_account(close_cpi)?;
        Ok(())
    }
}
