use anchor_lang::prelude::*;

#[constant]
pub const SHAIN_CONFIG_SEED: &[u8] = b"shain_config";

#[constant]
pub const SHAIN_SESSION_SEED: &[u8] = b"shain_session";

#[constant]
pub const SHAIN_TREASURY_SEED: &[u8] = b"shain_treasury";

pub const DEFAULT_SESSION_DURATION: i64 = 60 * 60 * 24;
pub const DEFAULT_SESSION_FEE: u64 = 1_000_000;
pub const DEFAULT_MIN_HOLDING: u64 = 10_000_000;

pub const MIN_SESSION_DURATION: i64 = 60;
pub const MAX_SESSION_DURATION: i64 = 60 * 60 * 24 * 30;
