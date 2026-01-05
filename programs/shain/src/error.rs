use anchor_lang::prelude::*;

#[error_code]
pub enum ShainError {
    #[msg("Signer is not the config authority")]
    Unauthorized,
    #[msg("Holder balance below minimum required to start a session")]
    InsufficientHolding,
    #[msg("Session is still active")]
    SessionStillActive,
    #[msg("Session has expired")]
    SessionExpired,
    #[msg("Session is not initialized")]
    SessionNotFound,
    #[msg("Session duration outside allowed range")]
    InvalidSessionDuration,
    #[msg("Token account owner does not match signer")]
    TokenAccountOwnerMismatch,
    #[msg("Token account mint does not match SHAIN mint")]
    TokenAccountMintMismatch,
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("Numeric cast failure")]
    CastError,
    #[msg("Treasury ATA does not match expected derivation")]
    TreasuryAtaMismatch,
}
