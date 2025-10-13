use anchor_lang::prelude::*;
use crate::state::constants::GAME_ACCOUNT_SEED;

pub fn get_signer_seeds<'a>(
    player_x: &'a Pubkey,
    created_at_le_bytes: &'a [u8],
    bump: &'a [u8],
) -> [&'a [u8]; 4] {
    [
        GAME_ACCOUNT_SEED,
        player_x.as_ref(),
        created_at_le_bytes,
        bump,
    ]
}