use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("The Price Is Invalid")]
    InvalidPrice,
    #[msg("Below Minimum Health Factor")]
    BelowMinimumHealthFactor,
    #[msg("Above Minimum Health Factor, Cannot Liquidate a healthy account")]
    AboveMinimumHealthFactor,
}
