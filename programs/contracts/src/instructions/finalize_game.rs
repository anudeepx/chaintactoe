use crate::errors::*;
use crate::events::*;
use crate::state::constants::*;
use crate::state::game::Game;
use crate::utils::*;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
pub struct FinalizeGame<'info> {
    #[account(
        mut,
        constraint = game.player_x == player_x.key() @ TicTacToeError::PlayerNotMatched,
        constraint = game.player_o == player_o.key() @ TicTacToeError::PlayerNotMatched,
    )]
    pub game: Account<'info, Game>,

    /// CHECK: Validated through game.player_x constraint
    #[account(mut)]
    pub player_x: AccountInfo<'info>,

    /// CHECK: Validated through game.player_o constraint
    #[account(mut)]
    pub player_o: AccountInfo<'info>,

    pub caller: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<FinalizeGame>) -> Result<()> {
    let game = &mut ctx.accounts.game;
    let now = Clock::get()?.unix_timestamp;

    // Validate game state
    require!(!game.is_finalized()?, TicTacToeError::GameAlreadyFinalized);
    require!(game.is_concluded()?, TicTacToeError::GameNotCompleted);

    let current_status = game.get_status()?;
    let total_pot = game.total_pot;
    
    require!(total_pot > 0, TicTacToeError::NoPot);

    // Calculate platform fee (5%)
    let fee = total_pot
        .checked_mul(5)
        .ok_or(TicTacToeError::Overflow)?
        .checked_div(100)
        .ok_or(TicTacToeError::Overflow)?;

    let remaining_pot = total_pot
        .checked_sub(fee)
        .ok_or(TicTacToeError::Overflow)?;

    let game_info = game.to_account_info();
    let ts_bytes = game.created_at.to_le_bytes();
    let bump_slice = [game.bump];

    let signer_seeds = get_signer_seeds(&game.player_x, &ts_bytes, &bump_slice);
    let signer = &[&signer_seeds[..]];

    match current_status {
        GameStatus::XWon => {
            // Pay winner
            let cpi_accounts = Transfer {
                from: game_info.clone(),
                to: ctx.accounts.player_x.to_account_info(),
            };
            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                cpi_accounts,
                signer,
            );
            transfer(cpi_ctx, remaining_pot)?;

            emit!(WinningsDistributed {
                game: game.key(),
                recipient: game.player_x,
                amount: remaining_pot,
                distributed_at: now
            });
            emit!(FeeCollected {
                game: game.key(),
                fee,
                collected_at: now
            });
            emit!(GameFinalized {
                game: game.key(),
                winner: game.player_x,
                closed_at: now
            });
        }
        GameStatus::OWon => {
            // Pay winner
            let cpi_accounts = Transfer {
                from: game_info.clone(),
                to: ctx.accounts.player_o.to_account_info(),
            };
            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                cpi_accounts,
                signer,
            );
            transfer(cpi_ctx, remaining_pot)?;

            emit!(WinningsDistributed {
                game: game.key(),
                recipient: game.player_o,
                amount: remaining_pot,
                distributed_at: now
            });
            emit!(FeeCollected {
                game: game.key(),
                fee,
                collected_at: now
            });
            emit!(GameFinalized {
                game: game.key(),
                winner: game.player_o,
                closed_at: now
            });
        }
        GameStatus::Draw => {
            // Refund both players (split pot)
            let refund_per_player = remaining_pot
                .checked_div(2)
                .ok_or(TicTacToeError::Overflow)?;

            // Refund player X
            let cpi_accounts_x = Transfer {
                from: game_info.clone(),
                to: ctx.accounts.player_x.to_account_info(),
            };
            let cpi_ctx_x = CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                cpi_accounts_x,
                signer,
            );
            transfer(cpi_ctx_x, refund_per_player)?;

            // Refund player O
            let cpi_accounts_o = Transfer {
                from: game_info.clone(),
                to: ctx.accounts.player_o.to_account_info(),
            };
            let cpi_ctx_o = CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                cpi_accounts_o,
                signer,
            );
            transfer(cpi_ctx_o, refund_per_player)?;

            emit!(FeeCollected {
                game: game.key(),
                fee,
                collected_at: now
            });
            emit!(GameFinalized {
                game: game.key(),
                winner: Pubkey::default(),
                closed_at: now
            });
        }
        _ => return Err(error!(TicTacToeError::GameNotCompleted)),
    }

    game.status = GameStatus::Finalized as u8;
    game.total_pot = 0;

    Ok(())
}