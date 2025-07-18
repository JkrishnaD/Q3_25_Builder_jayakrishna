use anchor_lang::prelude::*;

#[error_code]
pub enum MarketplaceErrors {
    #[msg("Custom error message")]
    CustomError,

    #[msg("Unautorized for this action")]
    Unauthorized,
}
