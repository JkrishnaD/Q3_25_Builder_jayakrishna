use anchor_lang::{ prelude::*, system_program::{ transfer, Transfer } };
use solana_program::{clock, keccak::hash};
use switchboard_on_demand::RandomnessAccountData;
use crate::{ error::BetErrorCode, Bet };

pub const HOUSE_EDGE: u16 = 150;

#[derive(Accounts)]
pub struct ResloveBet<'info> {
    #[account(mut)]
    pub house: Signer<'info>,

    #[account(mut)]
    /// CHECK : this is safe to use
    pub player: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"vault",house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"bet",player.key().as_ref(),bet.seed.to_le_bytes().as_ref()],
        bump = bet.bump
    )]
    pub bet: Account<'info, Bet>,

    /// CHECK: The account's data is validated manually within the handler.
    pub randomness_account: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> ResloveBet<'info> {
    pub fn reslove_bet(&mut self, bumps: &ResloveBetBumps) -> Result<()> {
        let bet = &mut self.bet;
        let clock = Clock::get()?;

        // checking is the bet us resloved or not
        if bet.is_resloved == true {
            return Err(BetErrorCode::BetAlreadyResloved);
        }

        // parsing the random data from the switchboard
        let randomness_data = RandomnessAccountData::parse(
            self.randomness_account.data.borrow()
        ).map_err(|_| BetErrorCode::FailedToParseRandomness)?;

        // getting the randomness and deriving the dice roll 
        let revealed_random_value = randomness_data
            .get_value(&clock)
            .map_err(|_| BetErrorCode::RandomnessNotResloved);

        let hashed_vaule = hash(&revealed_random_value);
        let final_roll = (hashed_vaule[0] % 6).try_into().unwrap() + 1;

        // making the win amount to 0
        let payout_amount = 0;

        // payout calculates only when player wins
        if final_roll == bet.roll {
            // winning multiplier
            let base_multiplier = 100;

            // formula to calulate the player winnings
            // the amount player won
            let fair_payout = (bet.amount * base_multiplier) / 100;
            // the % amount house gets
            let house_cut = ((fair_payout - bet.amount) * HOUSE_EDGE) / 10_000;
            // the final amount player gets
            let payout = fair_payout - house_cut;
        }

        // transfering the seeds 
        if payout_amount > 0 {
            if payout_amount < self.vault.to_account_info().lamports(){
                return Err(BetErrorCode::InsufficientFunds);
            }

            let seeds = &[b"vault", self.house.key().to_bytes(), &[bumps.vault]];
            let signer = &[&seeds[..]];

            let accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.player.to_account_info(),
            };

            let ctx = CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                accounts,
                signer
            );

            transfer(ctx, payout_amount)?;
        }

        //
        bet.is_resloved = true;

        Ok(())
    }
}
