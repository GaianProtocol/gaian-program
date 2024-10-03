use anchor_lang::prelude::*;

#[account]
pub struct Gaian {
    pub bump: u8,
    pub authority: Pubkey,
    pub mint: Option<Pubkey>, // None if using sol
    pub pt_mint: Pubkey,
    pub yt_mint: Pubkey,
    pub expiration_time: u64,
}

impl Space for Gaian {
    const INIT_SPACE: usize = 8 + 1000;
}

#[account]
pub struct SolVault {}

impl Space for SolVault {
    const INIT_SPACE: usize = 8;
}
