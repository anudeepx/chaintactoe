use crate::state::game::Game;
use crate::state::constants::{GameStatus, PlayerTurn, BOARD_SIZE};
use crate::errors::TicTacToeError;
use crate::events::{MoveMade, GameConcluded};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(position: u8)]
pub struct MakeMove<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    pub player: Signer<'info>,
}


pub fn handler(ctx: Context<MakeMove>, position: u8) -> Result<()> {
    let game = &mut ctx.accounts.game;
    let player = &ctx.accounts.player;
    let now = Clock::get()?.unix_timestamp;


    require!(game.is_active()?, TicTacToeError::GameNotActive);
    require!(position < BOARD_SIZE as u8, TicTacToeError::InvalidMovePosition);

    let is_player_x = player.key() == game.player_x;
    let is_player_o = player.key() == game.player_o;

    let (expected_turn_status, player_marker) = if is_player_x {
        (GameStatus::InProgressX, PlayerTurn::PlayerX)
    } else if is_player_o {
        (GameStatus::InProgressO, PlayerTurn::PlayerO)
    } else {
        return Err(error!(TicTacToeError::PlayerNotMatched));
    };

    require!(game.get_status()? == expected_turn_status, TicTacToeError::NotPlayersTurn);

    let idx = position as usize;
    require!(game.board[idx] == PlayerTurn::Empty as u8, TicTacToeError::CellOccupied);

    game.board[idx] = player_marker as u8;

    let win_conditions: [[usize; 3]; 8] = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8], // rows
        [0, 3, 6], [1, 4, 7], [2, 5, 8], // cols
        [0, 4, 8], [2, 4, 6],            // diags
    ];

    let mut new_status = GameStatus::Draw;
    let mut is_win = false;

    for condition in win_conditions.iter() {
        let a = condition[0];
        let b = condition[1];
        let c = condition[2];

        if game.board[a] == player_marker as u8 &&
           game.board[a] == game.board[b] &&
           game.board[b] == game.board[c] {
            new_status = if is_player_x { GameStatus::XWon } else { GameStatus::OWon };
            is_win = true;
            break;
        }
    }

    if is_win {
        game.status = new_status as u8;
        let final_status = if is_player_x { "XWon" } else { "OWon" }.to_string();
        emit!(GameConcluded { game: game.key(), final_status, concluded_at: now });
    } else if !game.board.contains(&(PlayerTurn::Empty as u8)) {
        game.status = GameStatus::Draw as u8;
        emit!(GameConcluded { game: game.key(), final_status: "Draw".to_string(), concluded_at: now });
    } else {
        game.switch_turn()?;
    }

    game.last_move_ts = now;

    emit!(MoveMade {
        game: game.key(),
        player: player.key(),
        position,
        made_at: now,
    });

    Ok(())
}