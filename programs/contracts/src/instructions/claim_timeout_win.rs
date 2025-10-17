use crate::errors::TicTacToeError;
use crate::events::GameConcluded;
use crate::state::constants::GameStatus;
use crate::state::game::Game;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClaimTimeoutWin<'info> {
    #[account(
        mut,
        constraint = game.player_x == player_x.key() @ TicTacToeError::PlayerNotMatched,
        constraint = game.player_o == player_o.key() @ TicTacToeError::PlayerNotMatched,
    )]
    pub game: Account<'info, Game>,

    /// CHECK: Validated through constraint
    pub player_x: AccountInfo<'info>,

    /// CHECK: Validated through constraint
    pub player_o: AccountInfo<'info>,

    pub claimer: Signer<'info>,
}

pub fn handler(ctx: Context<ClaimTimeoutWin>) -> Result<()> {
    let game = &mut ctx.accounts.game;
    let now = Clock::get()?.unix_timestamp;

    // Ensure game is active
    require!(game.is_active()?, TicTacToeError::GameNotActive);

    // Calculate time elapsed since last move
    let time_elapsed = now
        .checked_sub(game.last_move_ts)
        .ok_or(TicTacToeError::Overflow)?;

    // Check if timeout has occurred
    require!(
        time_elapsed >= game.timeout_seconds as i64,
        TicTacToeError::TimeoutNotReached
    );

    // Determine who timed out and declare the other player winner
    let current_status = game.get_status()?;
    let (winner_status, winner_pubkey) = match current_status {
        GameStatus::InProgressX => {
            // Player X's turn but timed out, O wins
            require!(
                ctx.accounts.claimer.key() == game.player_o,
                TicTacToeError::PlayerNotMatched
            );
            (GameStatus::OWon, game.player_o)
        }
        GameStatus::InProgressO => {
            // Player O's turn but timed out, X wins
            require!(
                ctx.accounts.claimer.key() == game.player_x,
                TicTacToeError::PlayerNotMatched
            );
            (GameStatus::XWon, game.player_x)
        }
        _ => return Err(error!(TicTacToeError::GameNotActive)),
    };

    game.status = winner_status as u8;

    emit!(GameConcluded {
        game: game.key(),
        final_status: format!("{:?}", winner_status),
        concluded_at: now,
    });

    msg!("Game concluded by timeout. Winner: {}", winner_pubkey);

    Ok(())
}