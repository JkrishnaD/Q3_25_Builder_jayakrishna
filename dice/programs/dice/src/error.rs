use anchor_lang::prelude::*;

#[error_code]
pub enum BetErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Bet already places")]
    BetAlreadyPlaced,
    #[msg("Bet is already being resloved")]
    BetAlreadyResloved,
    #[msg("Failed to pare the randomness")]
    FailedToParseRandomness,
    #[msg("Randomness is not resloved yet!")]
    RandomnessNotResloved,
    #[msg("Randomness is expired!")]
    RandomnessExpired,
    #[msg("Insuffient funds with the house")]
    InsufficientFunds
}
