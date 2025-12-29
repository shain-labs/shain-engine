//! Shain on-chain engine.
//!
//! Opens a time-boxed, holder-gated session on a Solana wallet. Dapps can
//! call [`gated_action`] to gate any downstream instruction behind an active
//! session, turning a noisy public signer into an uninteresting target for
//! copy-trade and sniper infrastructure for the duration of the session.

#![allow(clippy::result_large_err)]

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use error::*;
pub use instructions::*;
pub use state::*;

declare_id!("2T1Qs7f2hiy1sUQBWC7226xhXvCees97UfeqReRrnE66");

#[program]
pub mod shain {
    use super::*;

    /// Bootstrap the config PDA and treasury ATA. Called once by the
    /// authority.
    pub fn initialize(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
        instructions::initialize::handler(ctx, params)
    }

    /// Holder opens a new session. Charges the session fee, verifies the
    /// caller holds at least `min_holding` tokens and records a 24 hour
    /// window for the caller.
    pub fn start_session(ctx: Context<StartSession>) -> Result<()> {
        instructions::start_session::handler(ctx)
    }

    /// Integration hook. Downstream dapps CPI into this instruction (or
    /// gate their own ix on a successful return) to prove that the caller
    /// is inside an active session.
    pub fn gated_action(ctx: Context<GatedAction>, tag: u64) -> Result<()> {
        instructions::gated_action::handler(ctx, tag)
    }

    /// Cleanup. Anyone can call after expiry — rent is refunded to the
    /// session owner and state is wiped so the next entry is fresh.
    pub fn close_session(ctx: Context<CloseSession>) -> Result<()> {
        instructions::close_session::handler(ctx)
    }
}
