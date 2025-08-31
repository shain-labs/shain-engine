use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use error::*;
pub use instructions::*;
pub use state::*;

declare_id!("2T1Qs7f2hiy1sUQBWC7226xhXvCees97UfeqReRrnE66");

#[program]
pub mod shain {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
        instructions::initialize::handler(ctx, params)
    }

    pub fn start_session(ctx: Context<StartSession>) -> Result<()> {
        instructions::start_session::handler(ctx)
    }

    pub fn gated_action(ctx: Context<GatedAction>, tag: u64) -> Result<()> {
        instructions::gated_action::handler(ctx, tag)
    }
}
