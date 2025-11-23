/**
 * Minimal end-to-end sample. Opens a session, fires a single gated
 * action and prints the resulting snapshot. Intended to be run against
 * devnet after `anchor deploy`.
 *
 * Usage: node --loader ts-node/esm ./basic_session.ts
 */

import { Connection, Keypair, PublicKey, clusterApiUrl } from "@solana/web3.js";
import { readFileSync } from "node:fs";

import { ShainClient, formatRemaining, snapshotSession } from "../sdk/src";

async function main(): Promise<void> {
  const rpcUrl = process.env.SHAIN_RPC_URL ?? clusterApiUrl("devnet");
  const programIdString =
    process.env.SHAIN_PROGRAM_ID ?? "5BXxVhThrj7irsqNRGzHG3y1CSap4u3HoBmkVHfs4CNx";
  const mintString = required("SHAIN_MINT");
  const keypairPath = process.env.SHAIN_KEYPAIR_PATH ?? `${homeDir()}/.config/solana/id.json`;

  const connection = new Connection(rpcUrl, "confirmed");
  const wallet = loadKeypair(keypairPath);
  const programId = new PublicKey(programIdString);
  const shainMint = new PublicKey(mintString);

  const client = new ShainClient({ connection, programId, wallet });

  console.log(`wallet:    ${wallet.publicKey.toBase58()}`);
  console.log(`program:   ${programId.toBase58()}`);
  console.log(`rpc:       ${rpcUrl}`);

  const userAta = client.ata(wallet.publicKey, shainMint);
  console.log(`ata:       ${userAta.toBase58()}`);

  const existing = await client.fetchSession();
  if (existing) {
    const snap = snapshotSession(existing);
    console.log(`existing:  ${snap.active ? "active" : "expired"} (${formatRemaining(snap)})`);
    if (snap.active) {
      console.log("session already open — exiting");
      return;
    }
  }

  console.log("opening session...");
  const opened = await client.startSession({
    shainMint,
    userTokenAccount: userAta,
  });
  console.log(`  signature: ${opened.signature}`);
  console.log(`  expires:   ${new Date(opened.expiresAt * 1000).toISOString()}`);

  console.log("firing gated_action(tag=1)...");
  const gated = await client.gatedAction({ tag: 1n });
  console.log(`  signature: ${gated.signature}`);
  console.log(`  actions:   ${gated.actionsCount}`);

  const after = await client.snapshotSession();
  if (after) {
    console.log(`remaining: ${formatRemaining(after)}`);
  }
}

function required(key: string): string {
  const value = process.env[key];
  if (!value) {
    throw new Error(`missing required env var ${key}`);
  }
  return value;
}

function loadKeypair(path: string): Keypair {
  const raw = JSON.parse(readFileSync(path, "utf8"));
  const bytes = Uint8Array.from(raw);
  return Keypair.fromSecretKey(bytes);
}

function homeDir(): string {
  return process.env.HOME ?? process.env.USERPROFILE ?? ".";
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
