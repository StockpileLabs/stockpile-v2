use anchor_lang::prelude::*;

#[error_code]
pub enum ProtocolError {
    #[msg("The Pyth account provided is invalid")]
    PythAccountInvalid,

    #[msg("Failed to load price feed from Pyth account")]
    PythPriceFeedLoadFailed,

    #[msg("Failed to price from Pyth price feed. Perhaps price was too old")]
    PythPriceFeedPriceFailed,

    #[msg("Provided SPL Mint not supported")]
    MintNotSupported,

    #[msg("The provided name string should be a maximum of 50 characters long")]
    NameTooLong,

    #[msg("This pool has already transferred the funds to the receiver")]
    ReleasedFunds,

    #[msg("This pool has already been cancelled")]
    PoolClosed,

    #[msg("This pool is still active")]
    PoolStillActive,

    #[msg("A pool can't be created with a start time that's passed")]
    PoolInvalidStart,

    #[msg("The pool has not begun its funding round yet")]
    PoolNotStarted,

    #[msg("The end date has already passed")]
    EndDatePassed,

    #[msg("An error occurred in the quadratic funding algorithm")]
    AlgorithmFailure,

    #[msg("This project is currently inactive.")]
    DeactivatedProject,

    #[msg("This project has been closed a registered admin.")]
    ClosedProject,

    #[msg("This key is not authorized to make changes to this account.")]
    NotAuthorized,
}