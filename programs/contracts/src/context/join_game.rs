use crate::state::game::Game;
use crate::state::constants::GameStatus;
use crate::errors::TicTacToeError;
use crate::events::GameJoined;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
pub struct JoinGame<'info> {
    #[account(mut)]
    pub player_o: Signer<'info>,
    #[account(mut, has_one = player_x @ TicTacToeError::PlayerNotMatched)]
    pub game: Account<'info, Game>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<JoinGame>) -> Result<()> {
    let game = &mut ctx.accounts.game;
    let player_o = &ctx.accounts.player_o;

    require!(game.get_status()? == GameStatus::Open, TicTacToeError::GameNotOpen);

    let wager = game.wager;

    game.player_o = player_o.key();
    game.status = GameStatus::InProgressX as u8;
    game.last_move_ts = Clock::get()?.unix_timestamp;

    let cpi_accounts = Transfer {
        from: player_o.to_account_info(),
        to: game.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);
    transfer(cpi_ctx, wager)?;

    game.total_pot = game.total_pot.checked_add(wager).ok_or(TicTacToeError::Overflow)?;


    emit!(GameJoined {
        game: game.key(),
        player_o: game.player_o,
        joined_at: game.last_move_ts,
    });

    Ok(())
}