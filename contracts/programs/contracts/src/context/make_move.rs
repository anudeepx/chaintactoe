use crate::state::game::Game;
use crate::errors::CustomError;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct MakeMove<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    pub player: Signer<'info>,
}

pub fn make_move(ctx: Context<MakeMove>, x: u8, y: u8) -> Result<()> {
    let game = &mut ctx.accounts.game;
    let player = &ctx.accounts.player;

    require!(game.status == 1 || game.status == 2, CustomError::GameNotActive);

    let is_player_x = player.key() == game.player_x;
    let is_player_o = player.key() == game.player_o;
    require!(is_player_x || is_player_o, CustomError::PlayerNotMatched);

    if game.status == 1 {
        require!(is_player_x, CustomError::NotPlayersTurn);
    } else {
        require!(is_player_o, CustomError::NotPlayersTurn);
    }

    require!(x < 3 && y < 3, CustomError::InvalidMove);
    let idx = (x as usize) * 3 + (y as usize);
    require!(game.board[idx] == 0, CustomError::CellOccupied);

    // make move
    game.board[idx] = if is_player_x { 1 } else { 2 };

    // check wins
    let win_conditions: [[usize; 3]; 8] = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8], // rows
        [0, 3, 6], [1, 4, 7], [2, 5, 8], // cols
        [0, 4, 8], [2, 4, 6],            // diags
    ];

    for condition in win_conditions.iter() {
        let a = condition[0];
        let b = condition[1];
        let c = condition[2];
        if game.board[a] != 0 &&
           game.board[a] == game.board[b] &&
           game.board[b] == game.board[c] {
            game.status = if game.board[a] == 1 { 3 } else { 4 }; // 3 -> X won, 4 -> O won
            // update last_move_ts
            game.last_move_ts = Clock::get()?.unix_timestamp;
            emit!(MoveMade {
                game: game.key(),
                player: player.key(),
                x,
                y,
                made_at: game.last_move_ts,
            });
            return Ok(());
        }
    }

    // draw?
    if !game.board.contains(&0) {
        game.status = 5; // draw
    } else {
        // switch turns
        game.status = if game.status == 1 { 2 } else { 1 };
    }

    game.last_move_ts = Clock::get()?.unix_timestamp;

    emit!(MoveMade {
        game: game.key(),
        player: player.key(),
        x,
        y,
        made_at: game.last_move_ts,
    });

    Ok(())
}
