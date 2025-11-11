use anyhow::Result;
use clap::Subcommand;

use crate::config::Context;
use crate::info;

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Dump the decoded `ShainConfig` PDA.
    Show,
    /// Print the runtime configuration derived from env / flags.
    Runtime,
}

pub async fn run(action: ConfigAction, ctx: Context) -> Result<()> {
    match action {
        ConfigAction::Show => show(&ctx).await,
        ConfigAction::Runtime => runtime(&ctx),
    }
}

async fn show(ctx: &Context) -> Result<()> {
    info!(
        "config show network={} program={}",
        ctx.network.as_str(),
        ctx.program_id
    );
    println!("# dry-run — call client.fetchConfig() via the SDK to load the real PDA");
    Ok(())
}

fn runtime(ctx: &Context) -> Result<()> {
    let payload = serde_json::json!({
        "network": ctx.network.as_str(),
        "rpc": ctx.rpc_url,
        "keypair_path": ctx.keypair_path.display().to_string(),
        "program_id": ctx.program_id,
    });
    println!("{}", serde_json::to_string_pretty(&payload)?);
    Ok(())
}
