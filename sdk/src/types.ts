import type { Connection, Keypair, PublicKey } from "@solana/web3.js";
import BN from "bn.js";

/**
 * Shape of the `ShainConfig` PDA. Matches the Anchor account
 * definition in `programs/shain/src/state.rs`.
 */
export interface ShainConfig {
  authority: PublicKey;
  shainMint: PublicKey;
  treasuryAta: PublicKey;
  sessionDuration: BN;
  sessionFee: BN;
  minHolding: BN;
  totalSessions: BN;
  totalFeesCollected: BN;
  bump: number;
  treasuryBump: number;
}

/**
 * Shape of the per-holder `ShainSession` PDA.
 */
export interface ShainSession {
  owner: PublicKey;
  startedAt: BN;
  expiresAt: BN;
  actionsCount: BN;
  totalSessions: BN;
  bump: number;
}

/**
 * Parameters accepted by the on-chain `initialize` instruction.
 *
 * All fields are optional; omitted ones fall back to the defaults baked
 * into the program (`DEFAULT_SESSION_DURATION_SECONDS` etc).
 */
export interface InitializeParams {
  sessionDuration?: number | bigint;
  sessionFee?: number | bigint;
  minHolding?: number | bigint;
}

/**
 * Connection + wallet configuration the SDK needs to build and send
 * transactions.
 */
export interface ShainClientOptions {
  connection: Connection;
  programId: PublicKey;
  wallet: Keypair | WalletLike;
  commitment?: "processed" | "confirmed" | "finalized";
}

/**
 * Minimal interface a wallet adapter must satisfy. Compatible with
 * Solana wallet adapter and raw `Keypair` instances.
 */
export interface WalletLike {
  publicKey: PublicKey;
  signTransaction?: <T>(tx: T) => Promise<T>;
  signAllTransactions?: <T>(txs: T[]) => Promise<T[]>;
}

/** Result of `ShainClient.startSession`. */
export interface StartSessionResult {
  signature: string;
  sessionPda: PublicKey;
  startedAt: number;
  expiresAt: number;
}

/** Result of `ShainClient.gatedAction`. */
export interface GatedActionResult {
  signature: string;
  tag: bigint;
  actionsCount: number;
}

/** Result of `ShainClient.closeSession`. */
export interface CloseSessionResult {
  signature: string;
  refundedLamports: number;
}

/**
 * Structured snapshot of a session as understood by the SDK. Returned
 * by `client.fetchSession` and used as the source of truth for any UI
 * that renders session state.
 */
export interface SessionSnapshot {
  owner: PublicKey;
  startedAt: number;
  expiresAt: number;
  active: boolean;
  remainingSeconds: number;
  actionsCount: number;
  lifetimeSessions: number;
}

/**
 * Input to `derivePdas`. Either the program id alone or a full client
 * may be provided — `ShainClient.pdas()` is the preferred entry
 * point for most consumers.
 */
export interface PdaDerivationInput {
  programId: PublicKey;
  holder?: PublicKey;
}

/** Output bag for `derivePdas`. */
export interface PdaDerivationOutput {
  config: PublicKey;
  configBump: number;
  treasury: PublicKey;
  treasuryBump: number;
  session?: PublicKey;
  sessionBump?: number;
}
