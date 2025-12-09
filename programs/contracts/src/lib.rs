use anchor_lang::prelude::*;

pub mod instructions;
pub mod errors;
pub mod state;
pub mod events;
pub mod utils;

use instructions::*;
use crate::state::constants::BOARD_SIZE;

declare_id!("FnvUdc6BXAPAC3dY2d3YEiP4L3pq4wTfD4mYrhNFKHVA");

#[program]
pub mod contracts {
    use super::*;

    pub fn create_game(ctx: Context<CreateGame>, wager: u64) -> Result<()> {
        instructions::create_game::handler(ctx, wager)
    }

    pub fn join_game(ctx: Context<JoinGame>) -> Result<()> {
        instructions::join_game::handler(ctx)
    }

    pub fn make_move(ctx: Context<MakeMove>, position: u8) -> Result<()> {
        require!(
            position < BOARD_SIZE as u8,
            errors::TicTacToeError::InvalidMovePosition
        );
        instructions::make_move::handler(ctx, position)
    }

    pub fn finalize_game(ctx: Context<FinalizeGame>) -> Result<()> {
        instructions::finalize_game::handler(ctx)
    }

    pub fn claim_timeout_win(ctx: Context<ClaimTimeoutWin>) -> Result<()> {
        instructions::claim_timeout_win::handler(ctx)
    }
}