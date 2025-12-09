use anchor_lang::prelude::*;
use crate::state::*;

pub fn initialize_game(ctx: Context<InitializeGame>, player_two: Pubkey) -> Result<()> {
    let game = &mut ctx.accounts.game;
    game.players = [ctx.accounts.player_one.key(), player_two];
    game.turn = 0;
    game.board = [[None; 3]; 3];
    game.state = GameState::Active;
    msg!("Game initialized! Player 1: {}, Player 2: {}", game.players[0], game.players[1]);
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(
        init,
        payer = player_one,
        space = 8 + Game::INIT_SPACE,
        seeds = [b"game", player_one.key().as_ref()],
        bump
    )]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player_one: Signer<'info>,
    pub system_program: Program<'info, System>,
}
