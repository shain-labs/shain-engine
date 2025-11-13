use anyhow::Result;
use clap::Args as ClapArgs;

use crate::config::Context;

#[derive(ClapArgs, Debug)]
pub struct Args {
    /// Optional holder pubkey. When present, the per-holder session PDA
    /// is included in the output.
    #[arg(long)]
    pub holder: Option<String>,

    /// Emit JSON on stdout instead of a human-readable summary.
    #[arg(long, default_value_t = false)]
    pub json: bool,
}

pub async fn run(args: Args, ctx: Context) -> Result<()> {
    // We deliberately use fixed PDA labels rather than deriving on-chain
    // here — the CLI should not require a network round-trip just to print
    // the addresses. The labels are computed by the SDK using the same
    // seed bytes, so they stay in lockstep with the program.
    let payload = serde_json::json!({
        "program_id": ctx.program_id,
        "seeds": {
            "config": "shain_config",
            "session": "shain_session",
            "treasury": "shain_treasury",
        },
        "holder": args.holder,
        "note": "use the TypeScript SDK (derivePdas) for the exact base58 addresses",
    });

    if args.json {
        println!("{}", serde_json::to_string_pretty(&payload)?);
    } else {
        println!("program:   {}", ctx.program_id);
        println!(
            "config:    PDA([b\"shain_config\"], {})",
            ctx.program_id
        );
        println!(
            "treasury:  PDA([b\"shain_treasury\"], {})",
            ctx.program_id
        );
        if let Some(ref h) = args.holder {
            println!(
                "session:   PDA([b\"shain_session\", {}], {})",
                h, ctx.program_id
            );
        } else {
            println!("session:   (provide --holder <pubkey> to derive)");
        }
    }
    Ok(())
}
