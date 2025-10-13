use crate::state::game::Game;
use crate::state::constants::*;
use crate::errors::TicTacToeError;
use crate::events::GameCreated;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
#[instruction(wager: u64)]
pub struct CreateGame<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = Game::LEN,
        seeds = [GAME_ACCOUNT_SEED, creator.key().as_ref(), Clock::get()?.unix_timestamp.to_le_bytes().as_ref()],
        bump
    )]
    pub game: Account<'info, Game>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateGame>, wager: u64) -> Result<()> {
    require!(wager > 0, TicTacToeError::WagerMustBePositive);

    let game = &mut ctx.accounts.game;
    let creator = &ctx.accounts.creator;

    game.player_x = creator.key();
    game.player_o = Pubkey::default();
    game.wager = wager;
    game.board = [PlayerTurn::Empty as u8; BOARD_SIZE];
    game.status = GameStatus::Open as u8;
    game.created_at = Clock::get()?.unix_timestamp;
    game.last_move_ts = game.created_at;
    game.timeout_seconds = DEFAULT_TIMEOUT_SECONDS;
    game.bump = ctx.bumps.game;

    let cpi_accounts = Transfer {
        from: creator.to_account_info(),
        to: game.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);
    transfer(cpi_ctx, wager)?;

    game.total_pot = game.total_pot.checked_add(wager).ok_or(TicTacToeError::Overflow)?;

    emit!(GameCreated {
        game: game.key(),
        player_x: game.player_x,
        wager: game.wager,
        created_at: game.created_at,
    });

    Ok(())
}