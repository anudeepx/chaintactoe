use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::TicTacToeError;

pub fn play(ctx: Context<Play>, row: u8, col: u8) -> Result<()> {
    let game = &mut ctx.accounts.game;

    require!(game.state == GameState::Active, TicTacToeError::GameAlreadyOver);
    require!(row < 3 && col < 3, TicTacToeError::InvalidPosition);

    let current_player_index = (game.turn % 2) as usize;
    require!(
        game.players[current_player_index] == ctx.accounts.player.key(),
        TicTacToeError::NotPlayersTurn
    );

    require!(
        game.board[row as usize][col as usize].is_none(),
        TicTacToeError::TileAlreadySet
    );

    game.board[row as usize][col as usize] = Some(Sign::from_turn(game.turn));
    game.turn = game.turn.checked_add(1).unwrap();

    if let Some(winner) = game.check_winner() {
        game.state = GameState::Won { winner: winner as u8 };
        msg!("Game won by player: {}", game.players[winner]);
    } else if game.turn == 9 {
        game.state = GameState::Tie;
        msg!("Game ended in a tie!");
    }

    Ok(())
}

#[derive(Accounts)]
pub struct Play<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    pub player: Signer<'info>,
}
