//! Tiny logging facade. Avoids pulling in `tracing` / `env_logger` so the
//! CLI binary stays small. Behaviour is level-gated and writes to stderr
//! so stdout remains clean for scripting.

use anyhow::Result;
use std::sync::atomic::{AtomicU8, Ordering};

const LEVEL_ERROR: u8 = 1;
const LEVEL_WARN: u8 = 2;
const LEVEL_INFO: u8 = 3;
const LEVEL_DEBUG: u8 = 4;
const LEVEL_TRACE: u8 = 5;

static CURRENT_LEVEL: AtomicU8 = AtomicU8::new(LEVEL_INFO);

pub fn init(level: Option<&str>) -> Result<()> {
    let level = match level.map(str::to_ascii_lowercase).as_deref() {
        Some("error") => LEVEL_ERROR,
        Some("warn") => LEVEL_WARN,
        Some("info") | None => LEVEL_INFO,
        Some("debug") => LEVEL_DEBUG,
        Some("trace") => LEVEL_TRACE,
        Some(other) => anyhow::bail!("unknown log level: {other}"),
    };
    CURRENT_LEVEL.store(level, Ordering::Relaxed);
    Ok(())
}

pub fn log(level: u8, prefix: &str, msg: &str) {
    if level <= CURRENT_LEVEL.load(Ordering::Relaxed) {
        eprintln!("[{prefix}] {msg}");
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::logging::log(3, "info", &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! warn_ {
    ($($arg:tt)*) => {
        $crate::logging::log(2, "warn", &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! error_ {
    ($($arg:tt)*) => {
        $crate::logging::log(1, "error", &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::logging::log(4, "debug", &format!($($arg)*));
    };
}
