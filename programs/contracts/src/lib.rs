use anchor_lang::prelude::*;

pub mod context;
pub mod state;
pub mod errors;
pub mod events;
pub mod utils;

use context::create_game::CreateGame;
use context::join_game::JoinGame;
use context::make_move::MakeMove;
use context::finalize_game::FinalizeGame;
use state::constants::BOARD_SIZE;

declare_id!("7FvH5N8zG2DPo3vkoY8L7c5Y6SPzdyoJUNCmPUUnHUNx");

#[program]
pub mod contracts {
    use super::*;

    pub fn create_game(ctx: Context<CreateGame>, wager: u64) -> Result<()> {
        context::create_game::handler(ctx, wager)
    }

    pub fn join_game(ctx: Context<JoinGame>) -> Result<()> {
        context::join_game::handler(ctx)
    }

    pub fn make_move(ctx: Context<MakeMove>, position: u8) -> Result<()> {
        if position >= BOARD_SIZE as u8 {
            return Err(error!(errors::TicTacToeError::InvalidMovePosition));
        }
        context::make_move::handler(ctx, position)
    }

    pub fn finalize_game(ctx: Context<FinalizeGame>) -> Result<()> {
        context::finalize_game::handler(ctx)
    }
}