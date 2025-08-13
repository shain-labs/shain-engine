use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ShainConfig {
    pub authority: Pubkey,
    pub shain_mint: Pubkey,
    pub treasury_ata: Pubkey,
    pub session_duration: i64,
    pub session_fee: u64,
    pub min_holding: u64,
    pub total_sessions: u64,
    pub total_fees_collected: u64,
    pub bump: u8,
    pub treasury_bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct ShainSession {
    pub owner: Pubkey,
    pub started_at: i64,
    pub expires_at: i64,
    pub actions_count: u64,
    pub total_sessions: u64,
    pub bump: u8,
}

impl ShainSession {
    pub fn is_active(&self, now: i64) -> bool {
        now < self.expires_at
    }
}
