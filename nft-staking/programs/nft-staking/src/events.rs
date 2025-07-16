use anchor_lang::prelude::*;

#[event]
pub struct InitializeConfigEvent{
    pub config: Pubkey,
    pub max_stake: u8,
    pub points_per_stake: u8,
    pub freeze_period: u32,
}

#[event]
pub struct InitializeUserEvent{
    pub points: u32,
    pub amount_staked: u8,
    pub user: Pubkey,
}

#[event]
pub struct StakeEvent{
    pub owner: Pubkey,
    pub nft_mint: Pubkey,
    pub amount_staked: u8,
    pub stake_at: i64,
    pub stake_account: Pubkey,
}

#[event]
pub struct UnstakeEvent{
    pub owner: Pubkey,
    pub nft_mint: Pubkey,
    pub amount_staked: u8,
    pub stake_account: Pubkey,
}