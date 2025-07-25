use anchor_lang::{ prelude::*, system_program::{ transfer, Transfer } };

use crate::{ error::BetErrorCode, Bet };

#[derive(Accounts)]
#[instruction(seed:u128)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    ///CHECK: This check is safe
    pub house: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"vault",house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        init,
        payer = player,
        space = 8 + Bet::INIT_SPACE,
        seeds = [b"bet", player.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    pub bet: Account<'info, Bet>,

    /// CHECK: The account's data is validated manually within the handler.
    pub randomness_account: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> PlaceBet<'info> {
    pub fn create_bet(
        &mut self,
        amount: u64,
        roll: u8,
        seed: u128,
        randomness_account:Pubkey,
        bumps: &PlaceBetBumps
    ) -> Result<()> {
        if
            self.bet.is_resolved == false &&
            self.bet.commit_slot != 0 &&
            self.bet.key() != Pubkey::default()
        {
            return Err(BetErrorCode::BetAlreadyPlaced.into());
        }

        let accounts = Transfer {
            from: self.player.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);

        transfer(ctx, amount)?;

        // setting the bet account attributes
        self.bet.set_inner(Bet {
            amount,
            player: self.player.key(),
            slot: Clock::get()?.slot,
            seed,
            roll,
            bump: bumps.bet,
            randomness_account,
            commit_slot: Clock::get()?.slot,
            is_resloved: false,
        });
        Ok(())
    }
}
