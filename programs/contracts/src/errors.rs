use anchor_lang::prelude::*;

#[error_code]
pub enum TicTacToeError {
    #[msg("The game is not open for joining. It is already in progress or completed.")]
    GameNotOpen,

    #[msg("The current player is not a participant in this game.")]
    PlayerNotMatched,

    #[msg("The game is not in an active state (InProgressX or InProgressO).")]
    GameNotActive,

    #[msg("It is not this player's turn.")]
    NotPlayersTurn,

    #[msg("The chosen move position is invalid (must be between 0 and 8).")]
    InvalidMovePosition,

    #[msg("The chosen cell on the board is already occupied.")]
    CellOccupied,

    #[msg("The game is not in a concluded state for finalization (Win or Draw).")]
    GameNotCompleted,

    #[msg("No pot exists to distribute.")]
    NoPot,

    #[msg("Arithmetic overflow occurred during calculation.")]
    Overflow,

    #[msg("Wager must be greater than zero.")]
    WagerMustBePositive,

    #[msg("Game has already been finalized/closed.")]
    GameAlreadyFinalized,

    #[msg("Invalid game status value stored on the account.")]
    InvalidGameStatus,

    #[msg("The timeout period has not been reached yet.")]
    TimeoutNotReached,

    #[msg("Cannot play against yourself.")]
    CannotPlaySelf,

    #[msg("Wager amount exceeds maximum allowed.")]
    WagerTooHigh,

    #[msg("Insufficient funds to cover wager.")]
    InsufficientFunds,
}