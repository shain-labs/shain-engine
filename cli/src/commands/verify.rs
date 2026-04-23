use anyhow::{Context as _, Result};
use clap::Args as ClapArgs;

use crate::config::Context;
use crate::info;

#[derive(ClapArgs, Debug)]
pub struct Args {
    /// Root URL of the site to cross-check.
    #[arg(long = "site", default_value = "https://shain.fun")]
    pub site: String,

    /// Emit JSON on stdout instead of a human-readable summary.
    #[arg(long, default_value_t = false)]
    pub json: bool,
}

pub async fn run(args: Args, ctx: Context) -> Result<()> {
    info!(
        "verify site={} program={} network={}",
        args.site,
        ctx.program_id,
        ctx.network.as_str()
    );

    // The CLI intentionally does not perform HTTP fetches to keep the
    // dependency footprint minimal. It prints the curl commands an
    // operator should run and what they should return — this doubles as
    // live operational documentation.
    let well_known = format!("{}/.well-known/shain.json", args.site.trim_end_matches('/'));
    let health = format!("{}/api/health", args.site.trim_end_matches('/'));

    let payload = serde_json::json!({
        "program_id": ctx.program_id,
        "network": ctx.network.as_str(),
        "surfaces": {
            "well_known": well_known,
            "health": health,
            "repo_well_known": "https://github.com/shain-labs/shain-engine/blob/main/README.md",
        },
        "expect": {
            "program_id_matches": "program_id in all four surfaces equals the CLI program id",
            "drift_is_block": "any disagreement blocks merge to main",
        },
    });

    if args.json {
        println!("{}", serde_json::to_string_pretty(&payload)?);
        return Ok(());
    }

    println!("# run these commands and confirm each returns the same program id:");
    println!(
        "curl -fsSL {well_known} | jq -r .program_id",
        well_known = well_known
    );
    println!(
        "curl -fsSL {health} | jq -r .program",
        health = health
    );
    println!(
        "grep -E '5BXxVhThrj7irsqNRGzHG3y1CSap4u3HoBmkVHfs4CNx' README.md"
    );
    println!("# CLI program id: {}", ctx.program_id);

    verify_local(&ctx.program_id).context("local consistency check failed")?;
    println!("local:   OK");
    Ok(())
}

fn verify_local(program_id: &str) -> Result<()> {
    anyhow::ensure!(
        !program_id.is_empty(),
        "program id must be set (via --program-id or SHAIN_PROGRAM_ID)"
    );
    anyhow::ensure!(
        program_id.len() >= 32 && program_id.len() <= 44,
        "program id looks wrong (len={}): expected a base58 encoding of 32 bytes",
        program_id.len()
    );
    Ok(())
}
