use anyhow::Result;
use clap::Args as ClapArgs;

use crate::config::Context;
use crate::info;

#[derive(ClapArgs, Debug)]
pub struct Args {
    /// The SHAIN mint that the treasury ATA is opened for.
    #[arg(long)]
    pub mint: String,

    /// Session duration in seconds. Omit to use the program default (86400).
    #[arg(long = "duration")]
    pub duration: Option<u64>,

    /// Session fee in token base units. Omit to use the program default.
    #[arg(long)]
    pub fee: Option<u64>,

    /// Minimum holder balance required to open a session.
    #[arg(long = "min-hold")]
    pub min_holding: Option<u64>,
}

pub async fn run(args: Args, ctx: Context) -> Result<()> {
    info!(
        "initialize network={} rpc={} program={} mint={} duration={:?} fee={:?} min_hold={:?}",
        ctx.network.as_str(),
        ctx.rpc_url,
        ctx.program_id,
        args.mint,
        args.duration,
        args.fee,
        args.min_holding
    );

    // The initialize instruction is intentionally not auto-sent. Broadcasting it
    // requires authority privileges and hitting the wrong RPC could create a
    // config PDA in the wrong cluster. Print the command the user should run
    // after reviewing the payload.
    println!("# dry-run — use the TypeScript SDK or anchor CLI to actually broadcast");
    println!("anchor run initialize -- \\");
    println!("  --mint {} \\", args.mint);
    if let Some(d) = args.duration {
        println!("  --duration {} \\", d);
    }
    if let Some(f) = args.fee {
        println!("  --fee {} \\", f);
    }
    if let Some(m) = args.min_holding {
        println!("  --min-hold {} \\", m);
    }
    println!("  --cluster {}", ctx.network.as_str());
    Ok(())
}
