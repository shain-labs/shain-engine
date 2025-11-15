//! shain-cli: command-line operator for the Shain on-chain engine.
//!
//! The binary is intentionally thin — it dispatches parsed arguments to
//! the subcommand modules under `commands/`. Each subcommand owns the
//! RPC calls it needs so that tests can exercise them in isolation.

use anyhow::Result;
use clap::Parser;

mod commands;
mod config;
mod logging;

use commands::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    logging::init(cli.log_level.as_deref())?;

    let ctx = config::Context::from_env_or_cli(&cli)?;
    commands::dispatch(cli.command, ctx).await
}
