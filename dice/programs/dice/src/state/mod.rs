use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Bet {
    pub amount: u64,
    pub player: Pubkey,
    pub slot: u64,
    pub roll: u8,
    pub bump: u8,
    pub seed: u128,
}
