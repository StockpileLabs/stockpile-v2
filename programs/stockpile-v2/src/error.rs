use anchor_lang::prelude::*;

#[error_code]
pub enum ProtocolError {
    #[msg("The Pyth account provided is invalid")]
    PythAccountInvalid,

    #[msg("Failed to load price feed from Pyth account")]
    PythPriceFeedLoadFailed,

    #[msg("Failed to price from Pyth price feed. Perhaps price was too old")]
    PythPriceFeedPriceFailed,

    #[msg("Failed to load Civic Pass credentials.")]
    CivicFailure,

    #[msg("Provided SPL Mint not supported")]
    MintNotSupported,

    #[msg("The provided name string should be a maximum of 50 characters long")]
    NameTooLong,

    #[msg("This pool has already transferred the funds to the receiver")]
    ReleasedFunds,

    #[msg("This project is not currently apart of this pool.")]
    NotInPool,

    #[msg("This pool has already been cancelled")]
    PoolClosed,

    #[msg("This pool is still active")]
    PoolStillActive,

    #[msg("A pool can't be created with a start time that's passed")]
    PoolInvalidStart,

    #[msg("The pool has not begun its funding round yet")]
    PoolNotStarted,

    #[msg("Tried to add a fundraiser to pool when config is set to Open")]
    MismatchedConfig,

    #[msg("The end date has already passed")]
    EndDatePassed,

    #[msg("Extend date is less than the current configured end date")]
    ExtendDateInvalid,

    #[msg("An error occurred in the quadratic funding algorithm")]
    AlgorithmFailure,

    #[msg("This project is currently inactive.")]
    DeactivatedProject,

    #[msg("This milestone is currently closed.")]
    ClosedMilestone,

    #[msg("This milestone is still open.")]
    OpenMilestone,

    #[msg("This milestone is being reconciled.")]
    MilestoneIsReconciling,

    #[msg("This project has been closed by a registered admin.")]
    ClosedProject,

    #[msg("This key is not authorized to make changes to this account.")]
    NotAuthorized,

    #[msg("This fundraiser is already entered in the current funding round.")]
    AlreadyEntered,

    #[msg("This project has already claimed their grant.")]
    AlreadyClaimed,
}