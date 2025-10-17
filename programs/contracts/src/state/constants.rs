use anchor_lang::prelude::*;

pub const GAME_ACCOUNT_SEED: &[u8] = b"game";
pub const BOARD_SIZE: usize = 9;

/// The default timeout period (in seconds) for a player to make a move (1 minute).
pub const DEFAULT_TIMEOUT_SECONDS: u64 = 60;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
#[derive(Debug)]
pub enum GameStatus {
    Open,
    /// 1. Game is in progress, it is player 'X's turn to move.
    InProgressX,
    InProgressO,
    XWon,
    OWon,
    Draw,
    /// Game is finalized and its pot has been withdrawn. Used to prevent double withdrawal.
    Finalized,
}

impl Default for GameStatus {
    fn default() -> Self {
        GameStatus::Open
    }
}

// Helper to allow conversion from u8 status to enum
impl TryFrom<u8> for GameStatus {
    type Error = ProgramError;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(GameStatus::Open),
            1 => Ok(GameStatus::InProgressX),
            2 => Ok(GameStatus::InProgressO),
            3 => Ok(GameStatus::XWon),
            4 => Ok(GameStatus::OWon),
            5 => Ok(GameStatus::Draw),
            6 => Ok(GameStatus::Finalized),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

/// Defines the possible occupants of a board cell.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum PlayerTurn {
    /// 0. Cell is empty.
    Empty,
    /// 1. Player 'X' has marked the cell.
    PlayerX,
    /// 2. Player 'O' has marked the cell.
    PlayerO,
}

impl Default for PlayerTurn {
    fn default() -> Self {
        PlayerTurn::Empty
    }
}