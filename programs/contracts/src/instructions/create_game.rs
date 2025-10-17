use crate::errors::TicTacToeError;
use crate::events::GameCreated;
use crate::state::constants::*;
use crate::state::game::Game;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

// Maximum wager: 10 SOL
const MAX_WAGER: u64 = 10_000_000_000;

#[derive(Accounts)]
#[instruction(wager: u64)]
pub struct CreateGame<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = Game::LEN,
        seeds = [
            GAME_ACCOUNT_SEED,
            creator.key().as_ref(),
            Clock::get().unwrap().unix_timestamp.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub game: Account<'info, Game>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateGame<'info> {
    fn validate_wager(&self, wager: u64) -> Result<()> {
        require!(wager > 0, TicTacToeError::WagerMustBePositive);
        require!(wager <= MAX_WAGER, TicTacToeError::WagerTooHigh);

        // Check creator has sufficient balance
        let creator_balance = self.creator.lamports();
        let required_balance = wager
            .checked_add(Game::LEN as u64)
            .ok_or(TicTacToeError::Overflow)?;

        require!(
            creator_balance >= required_balance,
            TicTacToeError::InsufficientFunds
        );

        Ok(())
    }
}

pub fn handler(ctx: Context<CreateGame>, wager: u64) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    ctx.accounts.validate_wager(wager)?;

    let game = &mut ctx.accounts.game;
    let creator = &ctx.accounts.creator;

    // Initialize game state
    game.player_x = creator.key();
    game.player_o = Pubkey::default();
    game.wager = wager;
    game.board = [PlayerTurn::Empty as u8; BOARD_SIZE];
    game.status = GameStatus::Open as u8;
    game.created_at = now;
    game.last_move_ts = now;
    game.timeout_seconds = DEFAULT_TIMEOUT_SECONDS;
    game.bump = ctx.bumps.game;
    game.total_pot = 0;

    // Transfer wager to game account
    let cpi_accounts = Transfer {
        from: creator.to_account_info(),
        to: game.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        cpi_accounts,
    );
    transfer(cpi_ctx, wager)?;

    game.total_pot = game
        .total_pot
        .checked_add(wager)
        .ok_or(TicTacToeError::Overflow)?;

    emit!(GameCreated {
        game: game.key(),
        player_x: game.player_x,
        wager: game.wager,
        created_at: game.created_at,
    });

    msg!(
        "Game created: {} | Wager: {} lamports | Creator: {}",
        game.key(),
        wager,
        creator.key()
    );

    Ok(())
}