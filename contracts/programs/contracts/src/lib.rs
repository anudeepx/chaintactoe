use anchor_lang::prelude::*;

pub mod context;
pub mod state;
pub mod errors;
pub mod events;

use context::*;

declare_id!("EVn9PXaLKVBE1SXcZjbcKd9bQiuN1LvGm1RdnT3zqb1S");

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
        context::make_move::handler(ctx, position)
    }

    pub fn finalize_game(ctx: Context<FinalizeGame>) -> Result<()> {
        context::finalize_game::handler(ctx)
    }

}