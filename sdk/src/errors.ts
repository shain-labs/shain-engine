/**
 * Typed SDK-level error. Use `reason` to branch on known failure modes;
 * the embedded cause preserves the original RPC or Anchor error.
 */
export type ShainErrorReason =
  | "HolderBalanceTooLow"
  | "SessionAlreadyActive"
  | "SessionExpired"
  | "SessionNotFound"
  | "Unauthorized"
  | "InvalidParameter"
  | "ProgramIdMismatch"
  | "RpcError"
  | "Timeout"
  | "Unknown";

/**
 * Reverse lookup from the numeric Anchor error codes emitted by the
 * on-chain program into SDK-level reason tags. The numbers match the
 * `#[error_code]` ordering in `programs/shain/src/error.rs` plus
 * Anchor's 6000 base offset.
 */
export const ANCHOR_ERROR_REASONS: Record<number, ShainErrorReason> = {
  6000: "Unauthorized",
  6001: "HolderBalanceTooLow",
  6002: "SessionAlreadyActive",
  6003: "SessionExpired",
  6004: "SessionNotFound",
  6005: "InvalidParameter",
};

export class ShainSdkError extends Error {
  public readonly reason: ShainErrorReason;
  public readonly cause?: unknown;

  constructor(reason: ShainErrorReason, message: string, cause?: unknown) {
    super(`[shain:${reason}] ${message}`);
    this.name = "ShainSdkError";
    this.reason = reason;
    this.cause = cause;
  }

  static fromAnchorCode(code: number, cause?: unknown): ShainSdkError {
    const reason = ANCHOR_ERROR_REASONS[code] ?? "Unknown";
    const message = `on-chain error ${code}`;
    return new ShainSdkError(reason, message, cause);
  }

  static wrap(err: unknown): ShainSdkError {
    if (err instanceof ShainSdkError) {
      return err;
    }

    const message = err instanceof Error ? err.message : String(err);

    const anchorCodeMatch = message.match(/(?:Error|error) (?:Code: )?(\d{4,5})/);
    if (anchorCodeMatch) {
      const code = Number.parseInt(anchorCodeMatch[1] ?? "", 10);
      if (!Number.isNaN(code)) {
        return ShainSdkError.fromAnchorCode(code, err);
      }
    }

    if (/timeout|timed out/i.test(message)) {
      return new ShainSdkError("Timeout", message, err);
    }

    if (/ECONNREFUSED|fetch failed|getaddrinfo/i.test(message)) {
      return new ShainSdkError("RpcError", message, err);
    }

    return new ShainSdkError("Unknown", message, err);
  }
}

/**
 * Runtime guard for throwing a structured error when a bigint-like input
 * is out of range. Used by the client entry points to defend against
 * accidentally handing the program an overflowed tag / fee.
 */
export function assertU64(value: bigint, context: string): void {
  if (value < 0n || value > 0xffffffffffffffffn) {
    throw new ShainSdkError(
      "InvalidParameter",
      `${context} must fit in a u64 (got ${value})`,
    );
  }
}

/** Runtime guard that rejects values outside an inclusive numeric range. */
export function assertInRange(
  value: number,
  min: number,
  max: number,
  context: string,
): void {
  if (value < min || value > max) {
    throw new ShainSdkError(
      "InvalidParameter",
      `${context} must be between ${min} and ${max} (got ${value})`,
    );
  }
}
