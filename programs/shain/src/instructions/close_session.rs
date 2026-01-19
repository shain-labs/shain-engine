use anchor_lang::prelude::*;

use crate::{
    constants::SHAIN_SESSION_SEED, error::ShainError, state::ShainSession,
};

#[derive(Accounts)]
pub struct CloseSession<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        close = user,
        seeds = [SHAIN_SESSION_SEED, user.key().as_ref()],
        bump = shain_session.bump,
        constraint = shain_session.owner == user.key() @ ShainError::Unauthorized,
    )]
    pub shain_session: Account<'info, ShainSession>,
}

pub fn handler(ctx: Context<CloseSession>) -> Result<()> {
    let clock = Clock::get()?;
    let session = &ctx.accounts.shain_session;
    require!(
        !session.is_active(clock.unix_timestamp),
        ShainError::SessionStillActive
    );

    msg!(
        "shain.close user={} lifetime_sessions={}",
        session.owner,
        session.total_sessions
    );
    Ok(())
}
