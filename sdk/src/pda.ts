import { PublicKey } from "@solana/web3.js";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  SHAIN_SEEDS,
  TOKEN_PROGRAM_ID,
} from "./constants";
import type { PdaDerivationInput, PdaDerivationOutput } from "./types";

/**
 * Derive the full set of PDAs used by the Shain program.
 *
 * If `holder` is provided the per-holder session PDA is included. When
 * only the program id is known (e.g. rendering treasury totals) the
 * session fields are omitted.
 */
export function derivePdas(input: PdaDerivationInput): PdaDerivationOutput {
  const { programId, holder } = input;

  const [config, configBump] = PublicKey.findProgramAddressSync(
    [SHAIN_SEEDS.config],
    programId,
  );

  const [treasury, treasuryBump] = PublicKey.findProgramAddressSync(
    [SHAIN_SEEDS.treasury],
    programId,
  );

  const output: PdaDerivationOutput = {
    config,
    configBump,
    treasury,
    treasuryBump,
  };

  if (holder) {
    const [session, sessionBump] = PublicKey.findProgramAddressSync(
      [SHAIN_SEEDS.session, holder.toBuffer()],
      programId,
    );
    output.session = session;
    output.sessionBump = sessionBump;
  }

  return output;
}

/**
 * Associated Token Account derivation identical to
 * `spl-associated-token-account`'s canonical formula. Exposed so the
 * SDK does not need to drag in the full `@solana/spl-token` dependency
 * tree at call sites that only need the address.
 */
export function associatedTokenAddress(
  owner: PublicKey,
  mint: PublicKey,
  tokenProgramId: PublicKey = TOKEN_PROGRAM_ID,
): PublicKey {
  const [address] = PublicKey.findProgramAddressSync(
    [owner.toBuffer(), tokenProgramId.toBuffer(), mint.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );
  return address;
}

/**
 * Convenience helper for the treasury ATA, which is owned by the
 * treasury PDA (not by any user wallet).
 */
export function treasuryAta(
  programId: PublicKey,
  mint: PublicKey,
): { ata: PublicKey; treasury: PublicKey; treasuryBump: number } {
  const { treasury, treasuryBump } = derivePdas({ programId });
  const ata = associatedTokenAddress(treasury, mint);
  return { ata, treasury, treasuryBump };
}

/**
 * Compare two PDA maps for exact equality. Used by the manifest parity
 * check in the CLI to catch website / repo drift before it lands on
 * main.
 */
export function pdasEqual(a: PdaDerivationOutput, b: PdaDerivationOutput): boolean {
  if (!a.config.equals(b.config)) return false;
  if (!a.treasury.equals(b.treasury)) return false;
  if (a.configBump !== b.configBump) return false;
  if (a.treasuryBump !== b.treasuryBump) return false;

  const aSession = a.session?.toBase58() ?? null;
  const bSession = b.session?.toBase58() ?? null;
  if (aSession !== bSession) return false;
  if ((a.sessionBump ?? null) !== (b.sessionBump ?? null)) return false;

  return true;
}
