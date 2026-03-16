//! Poll a session's state and print a human-readable summary. Compile
//! with the rest of the workspace (`cargo run -p shain-cli`) and call
//! the `session status` subcommand for the hosted version.
//!
//! Standalone build:
//!   rustc --edition 2021 examples/monitor_session.rs -o monitor

use std::env;
use std::process::ExitCode;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() -> ExitCode {
    let holder = env::args().nth(1).unwrap_or_else(|| "(wallet)".into());
    let expires_at = env::args()
        .nth(2)
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or_else(|| unix_now() + 86_400);

    eprintln!("monitoring holder={holder} expires_at={expires_at}");

    loop {
        let now = unix_now();
        let remaining = expires_at - now;

        if remaining <= 0 {
            eprintln!("session expired — run `shain session close --user {holder}`");
            return ExitCode::from(0);
        }

        eprintln!(
            "active · {} remaining",
            format_duration(remaining as u64)
        );

        sleep(Duration::from_secs(60));
    }
}

fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

fn format_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;

    if hours > 0 {
        format!("{hours}h {minutes:02}m")
    } else if minutes > 0 {
        format!("{minutes}m {seconds:02}s")
    } else {
        format!("{seconds}s")
    }
}
