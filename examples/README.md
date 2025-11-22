# Examples

Runnable samples that exercise the Shain program through the
TypeScript SDK and the Rust program API.

| File                    | Language  | What it shows                                               |
| ----------------------- | --------- | ----------------------------------------------------------- |
| `basic_session.ts`      | TS        | Open a session, call `gated_action`, close on expiry        |
| `gate_swap.ts`          | TS        | Bundle `start_session` with a dex swap in one transaction   |
| `monitor_session.rs`    | Rust      | Poll a session's state and print a human-readable summary   |

Install SDK deps before running:

```bash
cd ../sdk
npm install && npm run build
```

Then run:

```bash
# TypeScript samples
node --loader ts-node/esm ../examples/basic_session.ts

# Rust sample (assumes the CLI is built)
cargo run -p shain-cli -- --help
cargo run --manifest-path ../cli/Cargo.toml -- session status
```
