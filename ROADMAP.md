# Roadmap

## Shipped

- [x] Core engine crate with four on-chain instructions
  (`initialize`, `start_session`, `gated_action`, `close_session`)
- [x] PDA-derived config and per-holder session accounts
- [x] Treasury ATA with session-fee accrual
- [x] Anchor 1.0 + Solana 3.1 toolchain support
- [x] Nine litesvm tests covering expiry, double-start, insufficient
      holding, rent refund on cleanup
- [x] TypeScript SDK scaffold with PDA derivation helpers
- [x] Rust CLI for local session management
- [x] Devnet-ready `Anchor.toml` with per-cluster program IDs
- [x] Community health: `CONTRIBUTING`, `SECURITY`, `CODE_OF_CONDUCT`,
      issue / PR templates, dependabot, release workflow
- [x] Integration examples for gating a swap and monitoring session state

See [CHANGELOG.md](./CHANGELOG.md) for the per-release breakdown.
