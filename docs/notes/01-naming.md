# Naming conventions

- On-chain types use `Shain` prefixes (`ShainConfig`, `ShainSession`).
- PDA seed bytes are snake case: `shain_config`, `shain_session`, `shain_treasury`.
- TypeScript types mirror Rust field names in camelCase (`sessionFee`, `minHolding`).
- Instruction discriminators are derived by Anchor; do not hand-assemble them.
