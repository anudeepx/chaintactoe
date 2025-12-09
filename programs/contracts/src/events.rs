use anchor_lang::prelude::*;

#[event]
pub struct GameCreated {
    pub game: Pubkey,
    pub player_x: Pubkey,
    pub wager: u64,
    pub created_at: i64,
}

#[event]
pub struct GameJoined {
    pub game: Pubkey,
    pub player_o: Pubkey,
    pub joined_at: i64,
}

#[event]
pub struct MoveMade {
    pub game: Pubkey,
    pub player: Pubkey,
    pub position: u8,
    pub made_at: i64,
}

#[event]
pub struct GameConcluded {
    pub game: Pubkey,
    pub final_status: String, // "XWon" | "OWon" | "Draw"
    pub concluded_at: i64,
}

#[event]
pub struct GameFinalized {
    pub game: Pubkey,
    pub winner: Pubkey, // Pubkey::default() for Draw/Error
    pub closed_at: i64,
}

#[event]
pub struct FeeCollected {
    pub game: Pubkey,
    pub fee: u64,
    pub collected_at: i64,
}

#[event]
pub struct WinningsDistributed {
    pub game: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
    pub distributed_at: i64,
}