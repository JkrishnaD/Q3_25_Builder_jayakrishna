use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ChallengeAccount {
    pub user: Pubkey, // the user who created the challenge
    pub wakeup_time: u64, // the time when the user wants to wake up
    pub stake_amount: u64, // the amount user wants to stake in
    pub vault_ata: Pubkey, // associated token account for the staked tokens
    pub is_active: bool, // whether the challenge is active or not
    pub mint: Pubkey, // mint of the token which is staked
    pub vault: Pubkey, // vault where the staked token are kept
    pub start_time: u64, // time when challenge was started
    pub end_time: u64, // time when challenge will end
    pub last_check_time: u64, // last time when the challenge was checked
    pub completed_days: u64, // the number of days completed in the challenge
    pub total_days: u64, // the total number of days in the challenge
    pub bump: u8, // bump seed for PDA
}

#[account]
#[derive(InitSpace)]
pub struct Treasury {
    pub authority: Pubkey, // authority of the treasury
    pub treasury_ata: Pubkey, // associated token account for BONK treasury
    pub bonk_mint: Pubkey, // mint of the BONK token
    pub total_collected: u64, // total amount collected in the treasury
    pub bump: u8, // bump seed for PDA
}
