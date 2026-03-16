# API reference

## On-chain instructions

| Name              | Signature                                | Caller    | Effect                                    |
| ----------------- | ---------------------------------------- | --------- | ----------------------------------------- |
| `initialize`      | `initialize(params: InitializeParams)`   | authority | Creates config + treasury ATA             |
| `start_session`   | `start_session()`                        | holder    | Charges fee, opens 24h window             |
| `gated_action`    | `gated_action(tag: u64)`                 | holder    | Asserts active session, increments count  |
| `close_session`   | `close_session()`                        | anyone    | Closes expired session, refunds rent      |

### `InitializeParams`

```rust
pub struct InitializeParams {
    pub session_duration: Option<i64>,
    pub session_fee: Option<u64>,
    pub min_holding: Option<u64>,
}
```

## SDK surface (`@shain/sdk`)

### `class ShainClient`

| Method                    | Returns                        | Notes                                        |
| ------------------------- | ------------------------------ | -------------------------------------------- |
| `pdas(holder?)`           | `PdaDerivationOutput`          | Deterministic PDA bundle                     |
| `ata(owner, mint)`        | `PublicKey`                    | Associated token address                     |
| `fetchConfig()`           | `Promise<ShainConfig \| null>` | Decodes the singleton config                 |
| `fetchSession(holder?)`   | `Promise<ShainSession \| null>` | Decodes a holder's session                   |
| `snapshotSession(holder?)`| `Promise<SessionSnapshot \| null>` | Human-friendly projection                    |
| `startSession(args)`      | `Promise<StartSessionResult>`  | Opens a 24h window                           |
| `gatedAction(args)`       | `Promise<GatedActionResult>`   | Increments counter while session is active   |
| `closeSession(args)`      | `Promise<CloseSessionResult>`  | Refunds rent after expiry                    |
| `predictExpiry()`         | `Promise<number \| null>`      | Reads config and computes `now + duration`   |

### Free functions

| Function                  | Purpose                                             |
| ------------------------- | --------------------------------------------------- |
| `derivePdas({programId, holder?})` | Compute the program's PDAs                 |
| `associatedTokenAddress(owner, mint)` | SPL-style ATA derivation                |
| `treasuryAta(programId, mint)` | Treasury PDA + ATA for the given mint          |
| `pdasEqual(a, b)`         | Structural comparison for drift checks              |
| `snapshotSession(account, nowSeconds?)` | Project a session into a UI snapshot     |
| `isSessionActive(snap, nowSeconds?)` | Clock-parameterised active check            |
| `formatRemaining(snap, nowSeconds?)` | `24h 15m` / `5m 03s` / `expired`            |
| `tagFromCallsite(module, fn)` | Stable `u64` tag for gated_action call sites    |
| `estimateExpiry(startedAt, duration)` | Predict `expires_at` without RPC           |
| `assertU64(value, ctx)`, `assertInRange(...)` | Input guards throwing `ShainSdkError` |

### Error taxonomy

`ShainSdkError.reason` is one of:

- `HolderBalanceTooLow`
- `SessionAlreadyActive`
- `SessionExpired`
- `SessionNotFound`
- `Unauthorized`
- `InvalidParameter`
- `ProgramIdMismatch`
- `RpcError`
- `Timeout`
- `Unknown`

Use `ShainSdkError.wrap(err)` to normalise an RPC / Anchor error from
an unrelated library into a typed SDK error.

## CLI

See [`cli/README.md`](../cli/README.md) for the subcommand table.
