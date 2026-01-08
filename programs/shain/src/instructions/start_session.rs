use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::{
    constants::{SHAIN_CONFIG_SEED, SHAIN_SESSION_SEED},
    error::ShainError,
    state::{ShainConfig, ShainSession},
};

#[derive(Accounts)]
pub struct StartSession<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [SHAIN_CONFIG_SEED],
        bump = shain_config.bump,
        has_one = shain_mint,
        has_one = treasury_ata,
    )]
    pub shain_config: Account<'info, ShainConfig>,

    pub shain_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = user_token_account.owner == user.key() @ ShainError::TokenAccountOwnerMismatch,
        constraint = user_token_account.mint == shain_mint.key() @ ShainError::TokenAccountMintMismatch,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub treasury_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + ShainSession::INIT_SPACE,
        seeds = [SHAIN_SESSION_SEED, user.key().as_ref()],
        bump,
    )]
    pub shain_session: Account<'info, ShainSession>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<StartSession>) -> Result<()> {
    let clock = Clock::get()?;
    let cfg = &ctx.accounts.shain_config;

    require!(
        ctx.accounts.user_token_account.amount >= cfg.min_holding,
        ShainError::InsufficientHolding
    );

    let session = &ctx.accounts.shain_session;
    require!(
        session.owner == Pubkey::default() || !session.is_active(clock.unix_timestamp),
        ShainError::SessionStillActive
    );

    if cfg.session_fee > 0 {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.key(),
            Transfer {
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.treasury_ata.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        token::transfer(cpi_ctx, cfg.session_fee)?;
    }

    let expires_at = clock
        .unix_timestamp
        .checked_add(cfg.session_duration)
        .ok_or(ShainError::Overflow)?;

    let session = &mut ctx.accounts.shain_session;
    let previous_sessions = session.total_sessions;
    session.owner = ctx.accounts.user.key();
    session.started_at = clock.unix_timestamp;
    session.expires_at = expires_at;
    session.actions_count = 0;
    session.total_sessions = previous_sessions
        .checked_add(1)
        .ok_or(ShainError::Overflow)?;
    session.bump = ctx.bumps.shain_session;

    let cfg = &mut ctx.accounts.shain_config;
    cfg.total_sessions = cfg
        .total_sessions
        .checked_add(1)
        .ok_or(ShainError::Overflow)?;
    cfg.total_fees_collected = cfg
        .total_fees_collected
        .checked_add(cfg.session_fee)
        .ok_or(ShainError::Overflow)?;

    msg!(
        "shain.start user={} expires_at={} total_sessions={}",
        session.owner,
        session.expires_at,
        session.total_sessions
    );
    Ok(())
}
