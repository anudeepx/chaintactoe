use anchor_lang::prelude::*;



declare_id!("FnvUdc6BXAPAC3dY2d3YEiP4L3pq4wTfD4mYrhNFKHVA");

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
