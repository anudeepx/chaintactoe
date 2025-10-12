use crate::state::game::*;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
pub struct CreateGame<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = Game::LEN,
        seeds = [b"game", creator.key().as_ref(), Clock::get().unwrap().unix_timestamp.to_le_bytes().as_ref()],
        bump
    )]
    pub game: Account<'info, Game>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateGame>, wager: u64) -> Result<()> {
    let game = &mut ctx.accounts.game;
    game.player_x = ctx.accounts.creator.key();
    game.wager = wager;
    game.board = [0; 9];
    game.turn = 1;
    game.status = 0; // game is open
    game.created_at = Clock::get().unwrap().unix_timestamp;
    game.last_move_ts = game.created_at;
    game.timeout_seconds = 120; // 2 minutes
    let bump: u8 = match ctx.bumps.get("game") {
        Some(b) => *b,
        None => {
            let ts_bytes = game.created_at.to_le_bytes();
            let (_pda, computed_bump) = Pubkey::find_program_address(
                &[b"game", ctx.accounts.creator.key.as_ref(), &ts_bytes],
                &crate::ID,
            );
            computed_bump
        }
    };
    game.bump = bump;

    let cpi_accounts = Transfer {
        from: ctx.accounts.creator.to_account_info(),
        to: ctx.accounts.game.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);
    transfer(cpi_ctx, wager)?;

    game.total_pot += wager;

    emit!(GameCreated {
        game: game.key(),
        player_x: game.player_x,
        wager: game.wager,
        created_at: game.created_at,
    });

    Ok(())
}
