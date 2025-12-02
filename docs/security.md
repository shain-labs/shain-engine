# Security

Shain is a narrow primitive. This document spells out what it does
protect against, what it does not, and the threat model we accept.

## In scope

- **Front-running protection during the session window.** While a
  holder's session is active, dapps gating on `gated_action` route their
  CPI through a path that is not observable by pre-confirmation mempool
  watchers.
- **Anti-sniper bootstrap.** Holders with a live session avoid the class
  of bot that listens to the public mempool and mirrors profitable
  wallets.
- **On-chain accountability.** Every session is represented by a PDA
  with a deterministic lifecycle. There is no custodial state.

## Out of scope

- **Long-term anonymity.** Sessions expire after 24 hours. The feed
  returns. Shain is deliberately not a mixer.
- **Anonymity set guarantees.** We do not claim k-anonymity or any
  formal anonymity property. Correlation resistance during the session
  is the only claim.
- **Protection after expiry.** Once a session closes, subsequent
  transactions by the same wallet are visible as usual.
- **Protection of wallets that never held SHAIN.** The gate is
  holder-only by design.

## Trust assumptions

- The signer's private key is the only custody primitive. The program
  never takes ownership of user funds.
- The program's upgrade authority is discarded at mainnet deployment.
- The treasury PDA is owned by the program; session fees accrue to it
  but cannot be withdrawn by any admin key.
- The SHAIN mint authority is discarded after initial supply issuance.

## Audit status

Pre-deployment. No mainnet contract address is published here or
anywhere under our control until the program is deployed and an
independent audit has completed.

## Reporting

See [`/SECURITY.md`](../SECURITY.md) for the vulnerability disclosure
process. Do not file public issues for security problems.
