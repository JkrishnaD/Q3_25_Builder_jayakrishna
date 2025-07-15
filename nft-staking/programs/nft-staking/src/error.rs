use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Still nft is not frozen")]
    NotFrozen,
    #[msg("No NFTs staked")]
    NoStakes
}
