use crate::errors::*;
use crate::events::*;
use crate::utils::*;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, System};

#[derive(Accounts)]
pub struct FinalizeGame<'info> {
    #[account(
        mut,
        has_one = player_x @ TicTacToeError::PlayerNotMatched,
        has_one = player_o @ TicTacToeError::PlayerNotMatched,
        close = recipient, 
    )] 
    pub game: Account<'info, Game>,

    pub player: Signer<'info>,
    
    #[account(mut)]
    pub recipient: AccountInfo<'info>,

    #[account(mut)]
    pub payout_recipient: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<FinalizeGame>) -> Result<()> {
    let game = &mut ctx.accounts.game;
    let player = &ctx.accounts.player;
    let payout_recipient = &ctx.accounts.payout_recipient;
    let now = Clock::get()?.unix_timestamp;

    let current_status = game.get_status()?;
    require!(game.is_concluded()?, TicTacToeError::GameNotCompleted);
    require!(!game.is_finalized()?, TicTacToeError::GameAlreadyFinalized);
    
    let player_is_participant = player.key() == game.player_x || player.key() == game.player_o;
    require!(player_is_participant, TicTacToeError::PlayerNotMatched);

    let total_pot = game.total_pot;
    require!(total_pot > 0, TicTacToeError::NoPot);

    let fee = total_pot.checked_div(20).ok_or(TicTacToeError::Overflow)?; 
    let payout_for_winner = total_pot.checked_sub(fee).ok_or(TicTacToeError::Overflow)?;
    let game_info = game.to_account_info();

    let winner_pubkey = game.get_winner_pubkey()?;
    let is_draw = current_status == GameStatus::Draw;

    let ts_bytes = game.created_at.to_le_bytes();
    let bump_slice = [game.bump];

    let (distribution_amount, recipient_pubkey_for_event) = if is_draw { 
        let refund_amount = total_pot.checked_div(2).ok_or(TicTacToeError::Overflow)?;
        require!(payout_recipient.key() == game.player_x || payout_recipient.key() == game.player_o, TicTacToeError::PlayerNotMatched);
        (refund_amount, payout_recipient.key())
    } else { 
        require!(payout_recipient.key() == winner_pubkey, TicTacToeError::PlayerNotMatched);
        (payout_for_winner, winner_pubkey)
    };

    let signer_seeds = get_signer_seeds(
        &game.player_x,
        ts_bytes.as_ref(),
        bump_slice.as_ref(),
    );
    
    let signer_seeds = get_signer_seeds(
    &game.player_x,
    ts_bytes.as_ref(),
    bump_slice.as_ref(),
    );

    let signer = &[&signer_seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        cpi_accounts,
        signer,
    );
    
    transfer(cpi_ctx, distribution_amount)?;
    
    game.status = GameStatus::Finalized as u8;
    game.total_pot = 0;

    if !is_draw {
        emit!(FeeCollected { game: game.key(), fee, collected_at: now });
        emit!(WinningsDistributed { game: game.key(), recipient: winner_pubkey, amount: payout_for_winner, distributed_at: now });
        emit!(GameFinalized { game: game.key(), winner: winner_pubkey, closed_at: now });
    } else {
        emit!(GameFinalized { game: game.key(), winner: Pubkey::default(), closed_at: now });
    }

    Ok(())
}