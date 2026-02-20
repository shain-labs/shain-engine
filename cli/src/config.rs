//! Runtime configuration for the CLI.
//!
//! Precedence: explicit flag > environment variable > built-in default.
//! The struct is intentionally flat — the CLI does not do any dynamic
//! discovery (no keyring, no remote config).

use anyhow::{Context as _, Result};
use std::path::PathBuf;

use crate::commands::Cli;

#[derive(Debug, Clone)]
pub struct Context {
    pub rpc_url: String,
    pub network: Network,
    pub keypair_path: PathBuf,
    pub program_id: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Network {
    Localnet,
    Devnet,
    Mainnet,
}

impl Network {
    pub fn as_str(self) -> &'static str {
        match self {
            Network::Localnet => "localnet",
            Network::Devnet => "devnet",
            Network::Mainnet => "mainnet-beta",
        }
    }

    pub fn default_rpc(self) -> &'static str {
        match self {
            Network::Localnet => "http://127.0.0.1:8899",
            Network::Devnet => "https://api.devnet.solana.com",
            Network::Mainnet => "https://api.mainnet-beta.solana.com",
        }
    }
}

impl std::str::FromStr for Network {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "localnet" | "local" => Ok(Network::Localnet),
            "devnet" | "dev" => Ok(Network::Devnet),
            "mainnet" | "mainnet-beta" | "main" => Ok(Network::Mainnet),
            other => anyhow::bail!("unknown network: {other}"),
        }
    }
}

impl Context {
    pub fn from_env_or_cli(cli: &Cli) -> Result<Self> {
        let network = match cli.network.as_deref() {
            Some(s) => s.parse()?,
            None => std::env::var("SHAIN_NETWORK")
                .ok()
                .map(|s| s.parse::<Network>())
                .transpose()?
                .unwrap_or(Network::Devnet),
        };

        let rpc_url = cli
            .rpc
            .clone()
            .or_else(|| std::env::var("SHAIN_RPC_URL").ok())
            .unwrap_or_else(|| network.default_rpc().to_string());

        let keypair_path = resolve_keypair_path(cli.keypair.clone())?;

        let program_id = cli
            .program_id
            .clone()
            .or_else(|| std::env::var("SHAIN_PROGRAM_ID").ok())
            .unwrap_or_else(|| "5BXxVhThrj7irsqNRGzHG3y1CSap4u3HoBmkVHfs4CNx".to_string());

        Ok(Context {
            rpc_url,
            network,
            keypair_path,
            program_id,
        })
    }
}

fn resolve_keypair_path(explicit: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(path) = explicit {
        return Ok(path);
    }

    if let Ok(env_path) = std::env::var("SHAIN_KEYPAIR_PATH") {
        return Ok(PathBuf::from(expand_tilde(&env_path)));
    }

    let home = dirs::home_dir().context("unable to resolve $HOME")?;
    Ok(home.join(".config").join("solana").join("id.json"))
}

fn expand_tilde(raw: &str) -> String {
    if let Some(stripped) = raw.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(stripped).to_string_lossy().into_owned();
        }
    }
    raw.to_string()
}
