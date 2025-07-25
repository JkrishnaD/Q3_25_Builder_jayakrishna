use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Bet {
    pub amount: u64,
    pub player: Pubkey,
    pub slot: u64,
    pub seed: u128, // to allow player to have multiple bets
    pub roll: u8,
    pub bump: u8,

    pub randomness_account:Pubkey,
    pub commit_slot:u64,
    pub is_resloved:bool
}

