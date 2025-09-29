import { PublicKey } from "@solana/web3.js";

/**
 * Canonical byte strings used to derive the program's PDAs.
 *
 * These match the seeds declared in
 * `programs/shain/src/constants.rs`. They are re-exported as a frozen
 * object so consumers cannot mutate them in place.
 */
export const SHAIN_SEEDS = Object.freeze({
  config: Buffer.from("shain_config"),
  session: Buffer.from("shain_session"),
  treasury: Buffer.from("shain_treasury"),
});

/**
 * The canonical Shain devnet program id.
 *
 * Keep this string in lockstep with:
 *  - `programs/shain/src/lib.rs` `declare_id!(...)`
 *  - `Anchor.toml` `[programs.*]` sections
 *  - `.well-known/shain.json` `program_id`
 *  - the `program` field in `/api/health`
 *
 * If they drift the integrity check in `verifyManifestParity` will fail.
 */
export const DEFAULT_PROGRAM_ID = new PublicKey(
  "5BXxVhThrj7irsqNRGzHG3y1CSap4u3HoBmkVHfs4CNx",
);

/** Session window default (24 hours, in seconds). */
export const DEFAULT_SESSION_DURATION_SECONDS = 60 * 60 * 24;

/** Session fee default in token base units (1 SHAIN with 6 decimals). */
export const DEFAULT_SESSION_FEE_UNITS = 1_000_000n;

/** Minimum holding required to open a session (10 SHAIN with 6 decimals). */
export const DEFAULT_MIN_HOLDING_UNITS = 10_000_000n;

/** Associated Token program id. */
export const ASSOCIATED_TOKEN_PROGRAM_ID = new PublicKey(
  "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
);

/** SPL Token program id (classic). */
export const TOKEN_PROGRAM_ID = new PublicKey(
  "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
);

/** System program id. */
export const SYSTEM_PROGRAM_ID = new PublicKey("11111111111111111111111111111111");

/** Default RPC endpoints for each supported cluster. */
export const DEFAULT_RPC = Object.freeze({
  localnet: "http://127.0.0.1:8899",
  devnet: "https://api.devnet.solana.com",
  "mainnet-beta": "https://api.mainnet-beta.solana.com",
});

/** Numeric tag used in `gated_action` to distinguish instrumented call sites. */
export const GATED_ACTION_DEFAULT_TAG = 0n;
