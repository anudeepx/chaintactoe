use anchor_lang::prelude::*;

#[account]
pub struct Game {
    pub player_x: Pubkey,
    pub player_o: Pubkey,
    pub wager: u64,             // lamports per player
    pub board: [u8; 9],         // 0 empty, 1 X, 2 O
    pub turn: u8,               // 1 = X, 2 = O
    pub status: u8,             // 0 open, 1 X won, 2 O won, 3 draw
    pub total_pot: u64,         // total amount in the pot
    pub created_at: i64,
    pub last_move_ts: i64,
    pub timeout_seconds: u64,    // time allowed per move
    pub bump: u8,
}

impl Game {
    pub const LEN: usize = 8
        + 32 // player_x
        + 32 // player_o
        + 8  // wager
        + 8  // total_pot
        + 9  // board
        + 1  // turn
        + 1  // status
        + 8  // created_at
        + 8  // last_move_ts
        + 8  // timeout_seconds
        + 1  // bump
        + 7; // padding to round up
}