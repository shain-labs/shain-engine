import type { PublicKey } from "@solana/web3.js";
import BN from "bn.js";
import type { SessionSnapshot, ShainSession } from "./types";

/**
 * Snapshot a `ShainSession` account into a plain-JS object suitable
 * for rendering in a UI or passing across an IPC boundary.
 */
export function snapshotSession(account: ShainSession, nowSeconds?: number): SessionSnapshot {
  const expiresAt = account.expiresAt.toNumber();
  const startedAt = account.startedAt.toNumber();
  const now = typeof nowSeconds === "number" ? nowSeconds : Math.floor(Date.now() / 1000);
  const remaining = Math.max(0, expiresAt - now);
  const active = now < expiresAt && !account.owner.equals(DEFAULT_ZERO_PUBKEY);

  return {
    owner: account.owner,
    startedAt,
    expiresAt,
    active,
    remainingSeconds: remaining,
    actionsCount: account.actionsCount.toNumber(),
    lifetimeSessions: account.totalSessions.toNumber(),
  };
}

/**
 * Check whether a given session accepts a new `gated_action`.
 *
 * Separated from the snapshot because callers sometimes hold a cached
 * snapshot and want to re-evaluate it against a later wall-clock time.
 */
export function isSessionActive(session: SessionSnapshot, nowSeconds?: number): boolean {
  const now = typeof nowSeconds === "number" ? nowSeconds : Math.floor(Date.now() / 1000);
  return session.expiresAt > now;
}

/**
 * Format the remaining session window as a human-readable string.
 * Returns `"expired"` when the session has already ended.
 */
export function formatRemaining(session: SessionSnapshot, nowSeconds?: number): string {
  const now = typeof nowSeconds === "number" ? nowSeconds : Math.floor(Date.now() / 1000);
  const remaining = session.expiresAt - now;

  if (remaining <= 0) {
    return "expired";
  }

  const hours = Math.floor(remaining / 3600);
  const minutes = Math.floor((remaining % 3600) / 60);
  const seconds = remaining % 60;

  if (hours > 0) {
    return `${hours}h ${minutes.toString().padStart(2, "0")}m`;
  }
  if (minutes > 0) {
    return `${minutes}m ${seconds.toString().padStart(2, "0")}s`;
  }
  return `${seconds}s`;
}

/**
 * Produce a monotonically-increasing numeric tag that can be passed to
 * `gated_action`. Useful for dapps that want to label each gated call
 * site for off-chain analytics without allocating a u64 ID per call.
 */
export function tagFromCallsite(module: string, fn: string): bigint {
  const raw = `${module}:${fn}`;
  let hash = 0n;
  for (let i = 0; i < raw.length; i++) {
    hash = (hash * 31n + BigInt(raw.charCodeAt(i))) & 0xffffffffffffffffn;
  }
  return hash === 0n ? 1n : hash;
}

const DEFAULT_ZERO_PUBKEY = Object.freeze({
  equals(other: PublicKey): boolean {
    return other.toBase58() === "11111111111111111111111111111111";
  },
}) as unknown as PublicKey;

/**
 * Estimate the unix second at which a fresh session started right now
 * would expire, given the program-configured duration.
 */
export function estimateExpiry(
  startedAtSeconds: number,
  sessionDurationSeconds: number | BN,
): number {
  const duration =
    typeof sessionDurationSeconds === "number"
      ? sessionDurationSeconds
      : sessionDurationSeconds.toNumber();
  return startedAtSeconds + duration;
}
