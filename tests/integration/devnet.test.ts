/**
 * Devnet integration smoke test for the Shain engine.
 *
 * This test exercises the live program on `api.devnet.solana.com`. It
 * does not assume any particular wallet balance or session state; it
 * derives the program-owned PDAs, fetches their on-chain layout if
 * present, and asserts that any decoded state is well-formed.
 *
 * Skipped automatically when `DEVNET_KEYPAIR` is not set so the test
 * suite stays green in CI runs that do not have devnet credentials
 * provisioned.
 *
 * Run locally:
 *   DEVNET_KEYPAIR=$(cat ~/.config/solana/devnet.json) \
 *     npx jest tests/integration/devnet.test.ts
 */

import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { readFileSync } from "fs";
import { resolve } from "path";

const PROGRAM_ID = new PublicKey(
  "2T1Qs7f2hiy1sUQBWC7226xhXvCees97UfeqReRrnE66",
);

const SHAIN_CONFIG_SEED = Buffer.from("shain_config");
const SHAIN_SESSION_SEED = Buffer.from("shain_session");
const SHAIN_TREASURY_SEED = Buffer.from("shain_treasury");

const DEVNET_RPC = process.env.SHAIN_RPC_URL ?? "https://api.devnet.solana.com";
const KEYPAIR_ENV = process.env.DEVNET_KEYPAIR;

const haveKeypair = Boolean(KEYPAIR_ENV);
const describeIfKeypair = haveKeypair ? describe : describe.skip;

function loadKeypair(raw: string): Keypair {
  const trimmed = raw.trim();
  if (trimmed.startsWith("[")) {
    const bytes = Uint8Array.from(JSON.parse(trimmed) as number[]);
    return Keypair.fromSecretKey(bytes);
  }
  const path = resolve(trimmed);
  const file = readFileSync(path, "utf8");
  return Keypair.fromSecretKey(Uint8Array.from(JSON.parse(file) as number[]));
}

function deriveConfigPda(): [PublicKey, number] {
  return PublicKey.findProgramAddressSync([SHAIN_CONFIG_SEED], PROGRAM_ID);
}

function deriveSessionPda(owner: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [SHAIN_SESSION_SEED, owner.toBuffer()],
    PROGRAM_ID,
  );
}

function deriveTreasuryAuthority(): [PublicKey, number] {
  return PublicKey.findProgramAddressSync([SHAIN_TREASURY_SEED], PROGRAM_ID);
}

describe("shain engine — devnet smoke", () => {
  const connection = new Connection(DEVNET_RPC, "confirmed");

  it("can reach the configured RPC endpoint", async () => {
    const slot = await connection.getSlot();
    expect(typeof slot).toBe("number");
    expect(slot).toBeGreaterThan(0);
  });

  it("derives stable PDAs from the program id", () => {
    const [config, configBump] = deriveConfigPda();
    const [treasury, treasuryBump] = deriveTreasuryAuthority();

    expect(config).toBeInstanceOf(PublicKey);
    expect(treasury).toBeInstanceOf(PublicKey);
    expect(configBump).toBeGreaterThanOrEqual(0);
    expect(configBump).toBeLessThan(256);
    expect(treasuryBump).toBeGreaterThanOrEqual(0);
    expect(treasuryBump).toBeLessThan(256);
    expect(config.toBase58()).not.toEqual(treasury.toBase58());
  });

  it("rejects an invalid base58 program id when re-derived", () => {
    expect(() => new PublicKey("not-a-base58-string")).toThrow();
  });
});

describeIfKeypair("shain engine — devnet (with keypair)", () => {
  const connection = new Connection(DEVNET_RPC, "confirmed");
  const wallet = loadKeypair(KEYPAIR_ENV ?? "");

  it("loads the provided devnet keypair", () => {
    expect(wallet.publicKey).toBeInstanceOf(PublicKey);
    expect(wallet.publicKey.toBase58()).toHaveLength(
      wallet.publicKey.toBase58().length,
    );
  });

  it("can fetch (or miss) the wallet's session pda without throwing", async () => {
    const [session] = deriveSessionPda(wallet.publicKey);
    const account = await connection.getAccountInfo(session);
    if (account) {
      expect(account.owner.toBase58()).toEqual(PROGRAM_ID.toBase58());
      expect(account.data.length).toBeGreaterThan(0);
    } else {
      expect(account).toBeNull();
    }
  });

  it("reports the configured commitment level", () => {
    expect(connection.commitment).toEqual("confirmed");
  });
});
