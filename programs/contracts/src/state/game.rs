use anchor_lang::prelude::*;
use crate::state::constants::*;

#[account]
#[derive(Default)]
pub struct Game {
    pub player_x: Pubkey,
    pub player_o: Pubkey,
    pub wager: u64,
    pub board: [u8; BOARD_SIZE],
    pub total_pot: u64,
    pub status: u8,
    pub created_at: i64,
    pub last_move_ts: i64,
    pub timeout_seconds: u64,
    pub bump: u8,
}

impl Game {
    pub const LEN: usize = 8
        + 32 // player_x Pubkey
        + 32 // player_o Pubkey
        + 8  // wager u64
        + BOARD_SIZE // board [u8; 9]
        + 8  // total_pot u64
        + 1  // status u8 (for GameStatus)
        + 8  // created_at i64
        + 8  // last_move_ts i64
        + 8  // timeout_seconds u64
        + 1; // bump u8

    pub fn get_status(&self) -> Result<GameStatus> {
        let status = GameStatus::try_from(self.status)
            .map_err(|_| error!(crate::errors::TicTacToeError::InvalidGameStatus))?;
        Ok(status)
    }

    pub fn is_active(&self) -> Result<bool> {
        let status = self.get_status()?;
        Ok(matches!(status, GameStatus::InProgressX | GameStatus::InProgressO))
    }

    pub fn is_concluded(&self) -> Result<bool> {
        let status = self.get_status()?;
        Ok(matches!(status, GameStatus::XWon | GameStatus::OWon | GameStatus::Draw))
    }
    
    pub fn is_finalized(&self) -> Result<bool> {
        Ok(self.get_status()? == GameStatus::Finalized)
    }

    pub fn get_winner_pubkey(&self) -> Result<Pubkey> {
        match self.get_status()? {
            GameStatus::XWon => Ok(self.player_x),
            GameStatus::OWon => Ok(self.player_o),
            _ => Ok(Pubkey::default()),
        }
    }

    pub fn switch_turn(&mut self) -> Result<()> {
        match self.get_status()? {
            GameStatus::InProgressX => self.status = GameStatus::InProgressO as u8,
            GameStatus::InProgressO => self.status = GameStatus::InProgressX as u8,
            _ => return Err(error!(crate::errors::TicTacToeError::GameNotActive)),
        }
        Ok(())
    }
}