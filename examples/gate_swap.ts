/**
 * Bundle `start_session` (or `gated_action` if already live) with a
 * third-party dex swap instruction in a single transaction. This is
 * the pattern integrating dapps are expected to follow.
 *
 * The dex ix is loaded from a JSON blob via `SHAIN_DEX_IX_PATH` so
 * this example stays framework-agnostic.
 */

import {
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  TransactionInstruction,
  clusterApiUrl,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import { readFileSync } from "node:fs";

import { ShainClient, tagFromCallsite } from "../sdk/src";

async function main(): Promise<void> {
  const rpcUrl = process.env.SHAIN_RPC_URL ?? clusterApiUrl("devnet");
  const connection = new Connection(rpcUrl, "confirmed");

  const wallet = loadKeypair(
    process.env.SHAIN_KEYPAIR_PATH ?? `${homeDir()}/.config/solana/id.json`,
  );

  const client = new ShainClient({
    connection,
    programId: new PublicKey(
      process.env.SHAIN_PROGRAM_ID ?? "5BXxVhThrj7irsqNRGzHG3y1CSap4u3HoBmkVHfs4CNx",
    ),
    wallet,
  });

  const shainMint = new PublicKey(required("SHAIN_MINT"));
  const userAta = client.ata(wallet.publicKey, shainMint);

  const existing = await client.snapshotSession();
  const tag = tagFromCallsite("examples", "gate_swap");

  const ixs: TransactionInstruction[] = [];
  if (!existing?.active) {
    ixs.push(
      client.buildStartSessionIx({
        shainMint,
        userTokenAccount: userAta,
      }),
    );
  }
  ixs.push(client.buildGatedActionIx({ tag }));
  ixs.push(...loadDexInstructions());

  const tx = new Transaction().add(...ixs);
  const signature = await sendAndConfirmTransaction(connection, tx, [wallet], {
    commitment: "confirmed",
  });

  console.log(`bundle confirmed: ${signature}`);
  console.log(`tag:              ${tag}`);
  console.log(`ix count:         ${ixs.length}`);
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
  return Keypair.fromSecretKey(Uint8Array.from(raw));
}

function homeDir(): string {
  return process.env.HOME ?? process.env.USERPROFILE ?? ".";
}

function loadDexInstructions(): TransactionInstruction[] {
  const path = process.env.SHAIN_DEX_IX_PATH;
  if (!path) {
    console.warn("SHAIN_DEX_IX_PATH not set — sending session ix only");
    return [];
  }

  const raw = JSON.parse(readFileSync(path, "utf8")) as Array<{
    programId: string;
    keys: Array<{ pubkey: string; isSigner: boolean; isWritable: boolean }>;
    data: string;
  }>;

  return raw.map((ix) => ({
    programId: new PublicKey(ix.programId),
    keys: ix.keys.map((k) => ({
      pubkey: new PublicKey(k.pubkey),
      isSigner: k.isSigner,
      isWritable: k.isWritable,
    })),
    data: Buffer.from(ix.data, "base64"),
  }));
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
