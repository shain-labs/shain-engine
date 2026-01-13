# CPI integration pattern

Downstream dapps should gate their private call behind a CPI into
`gated_action`. The `tag: u64` parameter is opaque to Shain; the dapp
owns its meaning. Suggested convention:

- Reserve `0` for the uninstrumented default.
- Use `tagFromCallsite(module, fn)` from `@shain/sdk` for a stable
  deterministic value that does not require a registry.
- Log `(wallet, tag, signature)` in the dapp's own telemetry if session
  attribution matters for retention metrics.
