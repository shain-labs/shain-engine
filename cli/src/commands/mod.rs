//! Subcommand dispatch for the `shain` CLI.

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::config::Context;

pub mod config_show;
pub mod init;
pub mod pdas;
pub mod session;
pub mod verify;

#[derive(Parser, Debug)]
#[command(
    name = "shain",
    about = "Command-line operator for the Shain private session engine.",
    version
)]
pub struct Cli {
    /// RPC URL to target. Overrides `SHAIN_RPC_URL`.
    #[arg(long, global = true)]
    pub rpc: Option<String>,

    /// Solana cluster. Overrides `SHAIN_NETWORK`. One of `localnet`,
    /// `devnet`, `mainnet`.
    #[arg(long, global = true)]
    pub network: Option<String>,

    /// Path to the signing keypair. Overrides `SHAIN_KEYPAIR_PATH`.
    #[arg(long, global = true)]
    pub keypair: Option<PathBuf>,

    /// Program id override. Overrides `SHAIN_PROGRAM_ID`.
    #[arg(long = "program-id", global = true)]
    pub program_id: Option<String>,

    /// Log level (`error`, `warn`, `info`, `debug`, `trace`).
    #[arg(long = "log-level", global = true)]
    pub log_level: Option<String>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Bootstrap the config + treasury ATA. Called once by the authority.
    Init(init::Args),

    /// Per-wallet session management.
    Session {
        #[command(subcommand)]
        action: session::SessionAction,
    },

    /// Show the decoded `ShainConfig` PDA.
    Config {
        #[command(subcommand)]
        action: config_show::ConfigAction,
    },

    /// Compute and print the program's PDAs.
    Pdas(pdas::Args),

    /// Cross-verify site, API, well-known and program ids.
    Verify(verify::Args),
}

pub async fn dispatch(cmd: Command, ctx: Context) -> Result<()> {
    match cmd {
        Command::Init(args) => init::run(args, ctx).await,
        Command::Session { action } => session::run(action, ctx).await,
        Command::Config { action } => config_show::run(action, ctx).await,
        Command::Pdas(args) => pdas::run(args, ctx).await,
        Command::Verify(args) => verify::run(args, ctx).await,
    }
}
