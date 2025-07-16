use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Still nft is not frozen")]
    NotFrozen,
    #[msg("No NFTs staked")]
    NoStakes,
    #[msg("Invalid nft amount")]
    InvalidNftAmount,
    #[msg("This NFT is not staked by the current user.")]
    InvalidStakeOwner,
    #[msg("Still nft is not thawed.")]
    StillFrozen,
}
