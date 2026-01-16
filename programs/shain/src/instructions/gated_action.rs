use anchor_lang::prelude::*;

use crate::{
    constants::SHAIN_SESSION_SEED, error::ShainError, state::ShainSession,
};

#[derive(Accounts)]
pub struct GatedAction<'info> {
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [SHAIN_SESSION_SEED, user.key().as_ref()],
        bump = shain_session.bump,
        constraint = shain_session.owner == user.key() @ ShainError::Unauthorized,
    )]
    pub shain_session: Account<'info, ShainSession>,
}

pub fn handler(ctx: Context<GatedAction>, tag: u64) -> Result<()> {
    let clock = Clock::get()?;
    let session = &mut ctx.accounts.shain_session;

    require!(
        session.owner != Pubkey::default(),
        ShainError::SessionNotFound
    );
    require!(
        session.is_active(clock.unix_timestamp),
        ShainError::SessionExpired
    );

    session.actions_count = session
        .actions_count
        .checked_add(1)
        .ok_or(ShainError::Overflow)?;

    msg!(
        "shain.gated user={} expires_at={} tag={} actions={}",
        session.owner,
        session.expires_at,
        tag,
        session.actions_count
    );
    Ok(())
}
