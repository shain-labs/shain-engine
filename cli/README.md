# shain-cli

Command-line operator for the Shain private session engine. Wraps
the on-chain instructions in ergonomic subcommands so you can manage a
session without writing TypeScript.

## Build

```bash
cd shain-engine
cargo build --release -p shain-cli
./target/release/shain --help
```

## Subcommands

| Command                        | Description                                                     |
| ------------------------------ | --------------------------------------------------------------- |
| `shain init`                 | One-time bootstrap of the config + treasury ATA (authority)     |
| `shain session open`         | Open a new 24-hour session for the current wallet               |
| `shain session close`        | Close an expired session and refund rent                        |
| `shain session status`       | Print the session's current expiry and gated-action count       |
| `shain config show`          | Dump the decoded `ShainConfig` PDA                            |
| `shain pdas`                 | Print the deterministic PDAs for a program id and holder        |
| `shain verify`               | Compare site/API/well-known program ids against the program     |

## Environment

| Variable                    | Default                                          |
| --------------------------- | ------------------------------------------------ |
| `SHAIN_RPC_URL`           | `https://api.devnet.solana.com`                  |
| `SHAIN_KEYPAIR_PATH`      | `~/.config/solana/id.json`                       |
| `SHAIN_NETWORK`           | `devnet`                                         |
| `SHAIN_PROGRAM_ID`        | `5BXxVhThrj7irsqNRGzHG3y1CSap4u3HoBmkVHfs4CNx`   |
| `SHAIN_LOG_LEVEL`         | `info`                                           |

See the root-level `.env.example` for a template.
