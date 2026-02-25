use anyhow::Result;
use clap::{Args as ClapArgs, Subcommand};

use crate::config::Context;
use crate::info;

#[derive(Subcommand, Debug)]
pub enum SessionAction {
    /// Open a new 24h private session for the configured wallet.
    Open(OpenArgs),
    /// Close an expired session.
    Close(CloseArgs),
    /// Print the current session state.
    Status(StatusArgs),
}

#[derive(ClapArgs, Debug)]
pub struct OpenArgs {
    /// User token account holding the SHAIN balance.
    #[arg(long = "user-ata")]
    pub user_ata: Option<String>,

    /// Tag value for the first gated action after opening. Useful when a
    /// dapp wants to instrument that a session was bundled with a trade.
    #[arg(long)]
    pub warm_tag: Option<u64>,
}

#[derive(ClapArgs, Debug)]
pub struct CloseArgs {
    /// Owner of the session. Defaults to the signing wallet.
    #[arg(long)]
    pub user: Option<String>,
}

#[derive(ClapArgs, Debug)]
pub struct StatusArgs {
    /// Owner of the session. Defaults to the signing wallet.
    #[arg(long)]
    pub user: Option<String>,

    /// Emit JSON on stdout instead of a human-readable summary.
    #[arg(long, default_value_t = false)]
    pub json: bool,
}

pub async fn run(action: SessionAction, ctx: Context) -> Result<()> {
    match action {
        SessionAction::Open(args) => open(args, ctx).await,
        SessionAction::Close(args) => close(args, ctx).await,
        SessionAction::Status(args) => status(args, ctx).await,
    }
}

async fn open(args: OpenArgs, ctx: Context) -> Result<()> {
    info!(
        "session open network={} program={} user-ata={:?} warm-tag={:?}",
        ctx.network.as_str(),
        ctx.program_id,
        args.user_ata,
        args.warm_tag
    );

    println!("# dry-run — broadcast via the SDK:");
    println!("import {{ ShainClient }} from '@shain/sdk';");
    println!("const res = await client.startSession({{ shainMint, userTokenAccount }});");
    Ok(())
}

async fn close(args: CloseArgs, ctx: Context) -> Result<()> {
    info!(
        "session close network={} program={} user={:?}",
        ctx.network.as_str(),
        ctx.program_id,
        args.user
    );

    println!("# dry-run — broadcast via the SDK:");
    println!("await client.closeSession({{ user }});");
    Ok(())
}

async fn status(args: StatusArgs, ctx: Context) -> Result<()> {
    info!(
        "session status network={} program={} user={:?} json={}",
        ctx.network.as_str(),
        ctx.program_id,
        args.user,
        args.json
    );

    if args.json {
        let payload = serde_json::json!({
            "network": ctx.network.as_str(),
            "program": ctx.program_id,
            "user": args.user,
            "note": "status is a dry-run; fetch via the SDK for live data",
        });
        println!("{}", serde_json::to_string_pretty(&payload)?);
    } else {
        println!(
            "# dry-run — call client.snapshotSession({}) via the SDK",
            args.user.unwrap_or_else(|| "<wallet>".into())
        );
    }
    Ok(())
}
