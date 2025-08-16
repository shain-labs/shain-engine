use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::{
    constants::{
        DEFAULT_MIN_HOLDING, DEFAULT_SESSION_DURATION, DEFAULT_SESSION_FEE,
        MAX_SESSION_DURATION, MIN_SESSION_DURATION, SHAIN_CONFIG_SEED, SHAIN_TREASURY_SEED,
    },
    error::ShainError,
    state::ShainConfig,
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub shain_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        space = 8 + ShainConfig::INIT_SPACE,
        seeds = [SHAIN_CONFIG_SEED],
        bump,
    )]
    pub shain_config: Account<'info, ShainConfig>,

    /// CHECK: PDA that owns the treasury ATA; only signs via CPI.
    #[account(
        seeds = [SHAIN_TREASURY_SEED],
        bump,
    )]
    pub treasury_authority: UncheckedAccount<'info>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = shain_mint,
        associated_token::authority = treasury_authority,
    )]
    pub treasury_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InitializeParams {
    pub session_duration: Option<i64>,
    pub session_fee: Option<u64>,
    pub min_holding: Option<u64>,
}

pub fn handler(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
    let duration = params.session_duration.unwrap_or(DEFAULT_SESSION_DURATION);
    require!(
        (MIN_SESSION_DURATION..=MAX_SESSION_DURATION).contains(&duration),
        ShainError::InvalidSessionDuration
    );

    let cfg = &mut ctx.accounts.shain_config;
    cfg.authority = ctx.accounts.authority.key();
    cfg.shain_mint = ctx.accounts.shain_mint.key();
    cfg.treasury_ata = ctx.accounts.treasury_ata.key();
    cfg.session_duration = duration;
    cfg.session_fee = params.session_fee.unwrap_or(DEFAULT_SESSION_FEE);
    cfg.min_holding = params.min_holding.unwrap_or(DEFAULT_MIN_HOLDING);
    cfg.total_sessions = 0;
    cfg.total_fees_collected = 0;
    cfg.bump = ctx.bumps.shain_config;
    cfg.treasury_bump = ctx.bumps.treasury_authority;

    msg!(
        "shain.init authority={} mint={} duration={}s fee={} min_hold={}",
        cfg.authority,
        cfg.shain_mint,
        cfg.session_duration,
        cfg.session_fee,
        cfg.min_holding
    );
    Ok(())
}
