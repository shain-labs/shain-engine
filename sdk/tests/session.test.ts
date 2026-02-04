import { Keypair, PublicKey } from "@solana/web3.js";
import BN from "bn.js";
import {
  DEFAULT_PROGRAM_ID,
  SHAIN_SEEDS,
  associatedTokenAddress,
  derivePdas,
  estimateExpiry,
  formatRemaining,
  isSessionActive,
  pdasEqual,
  snapshotSession,
  tagFromCallsite,
  TOKEN_PROGRAM_ID,
} from "../src";
import { ShainSdkError, assertInRange, assertU64 } from "../src/errors";

describe("derivePdas", () => {
  it("returns stable config and treasury PDAs for a given program id", () => {
    const first = derivePdas({ programId: DEFAULT_PROGRAM_ID });
    const second = derivePdas({ programId: DEFAULT_PROGRAM_ID });
    expect(first.config.toBase58()).toEqual(second.config.toBase58());
    expect(first.treasury.toBase58()).toEqual(second.treasury.toBase58());
    expect(first.session).toBeUndefined();
  });

  it("derives a session PDA when a holder is provided", () => {
    const holder = Keypair.generate().publicKey;
    const pdas = derivePdas({ programId: DEFAULT_PROGRAM_ID, holder });
    expect(pdas.session).toBeDefined();
    expect(pdas.sessionBump).toBeGreaterThanOrEqual(0);
  });

  it("pdasEqual detects drift in any field", () => {
    const a = derivePdas({ programId: DEFAULT_PROGRAM_ID });
    const b = derivePdas({
      programId: new PublicKey("11111111111111111111111111111112"),
    });
    expect(pdasEqual(a, b)).toBe(false);
  });
});

describe("associatedTokenAddress", () => {
  it("produces the canonical ATA derivation", () => {
    const owner = Keypair.generate().publicKey;
    const mint = Keypair.generate().publicKey;
    const ata1 = associatedTokenAddress(owner, mint);
    const ata2 = associatedTokenAddress(owner, mint, TOKEN_PROGRAM_ID);
    expect(ata1.toBase58()).toEqual(ata2.toBase58());
  });
});

describe("session helpers", () => {
  const base = {
    owner: Keypair.generate().publicKey,
    startedAt: new BN(1_700_000_000),
    expiresAt: new BN(1_700_086_400),
    actionsCount: new BN(2),
    totalSessions: new BN(5),
    bump: 250,
  };

  it("snapshotSession reports active status when before expiry", () => {
    const snap = snapshotSession(base, 1_700_000_500);
    expect(snap.active).toBe(true);
    expect(snap.remainingSeconds).toBe(86_400 - 500);
    expect(snap.actionsCount).toBe(2);
  });

  it("snapshotSession reports inactive after expiry", () => {
    const snap = snapshotSession(base, 1_700_999_999);
    expect(snap.active).toBe(false);
    expect(snap.remainingSeconds).toBe(0);
  });

  it("isSessionActive follows the snapshot clock", () => {
    const snap = snapshotSession(base, 1_700_000_000);
    expect(isSessionActive(snap, 1_700_086_399)).toBe(true);
    expect(isSessionActive(snap, 1_700_086_400)).toBe(false);
  });

  it("formatRemaining emits a legible string", () => {
    const snap = snapshotSession(base, 1_700_000_000);
    expect(formatRemaining(snap, 1_700_000_000)).toMatch(/24h|23h 59m/);
    expect(formatRemaining(snap, 1_700_086_400)).toBe("expired");
  });

  it("tagFromCallsite is stable and non-zero", () => {
    const a = tagFromCallsite("dex", "swap");
    const b = tagFromCallsite("dex", "swap");
    expect(a).toEqual(b);
    expect(a).not.toEqual(0n);
  });

  it("estimateExpiry adds the duration correctly", () => {
    expect(estimateExpiry(1000, 3600)).toBe(4600);
    expect(estimateExpiry(1000, new BN(3600))).toBe(4600);
  });
});

describe("error guards", () => {
  it("assertU64 rejects negatives and overflow", () => {
    expect(() => assertU64(-1n, "ctx")).toThrow(ShainSdkError);
    expect(() => assertU64(0xffffffffffffffffn + 1n, "ctx")).toThrow(ShainSdkError);
    expect(() => assertU64(42n, "ctx")).not.toThrow();
  });

  it("assertInRange rejects values outside the inclusive range", () => {
    expect(() => assertInRange(0, 1, 10, "x")).toThrow(ShainSdkError);
    expect(() => assertInRange(11, 1, 10, "x")).toThrow(ShainSdkError);
    expect(() => assertInRange(5, 1, 10, "x")).not.toThrow();
  });

  it("ShainSdkError.wrap recognises anchor error codes", () => {
    const raw = new Error("Error Code: 6001");
    const err = ShainSdkError.wrap(raw);
    expect(err.reason).toBe("HolderBalanceTooLow");
    expect(err.cause).toBe(raw);
  });

  it("ShainSdkError.wrap falls back to RpcError for network failures", () => {
    const raw = new Error("fetch failed");
    const err = ShainSdkError.wrap(raw);
    expect(err.reason).toBe("RpcError");
  });
});

describe("SHAIN_SEEDS", () => {
  it("exposes the exact byte strings the program uses", () => {
    expect(SHAIN_SEEDS.config.toString()).toBe("shain_config");
    expect(SHAIN_SEEDS.session.toString()).toBe("shain_session");
    expect(SHAIN_SEEDS.treasury.toString()).toBe("shain_treasury");
  });
});
