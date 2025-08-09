use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2T1Qs7f2hiy1sUQBWC7226xhXvCees97UfeqReRrnE66");

#[program]
pub mod shain {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> { Ok(()) }
}
