# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.4.x   | :white_check_mark: |
| 0.3.x   | :x:                |
| < 0.3   | :x:                |

## Reporting a Vulnerability

Do not open public GitHub issues for security problems. Send a report to
`security@shain.fun` including:

1. A description of the vulnerability
2. Steps to reproduce or a proof-of-concept
3. The commit hash or release version affected
4. The impact you observed (what can an attacker do?)
5. Your preferred disclosure timeline

We will:

- Acknowledge receipt within 48 hours
- Provide an initial assessment within 5 business days
- Keep you updated on remediation progress
- Credit you in the advisory (unless you prefer to remain anonymous)

## Scope

In scope:

- The on-chain program in `programs/shain/`
- The TypeScript SDK in `sdk/`
- The CLI in `cli/`
- Official example code in `examples/`

Out of scope:

- The marketing website (`shain.fun`)
- Third-party integrations that depend on Shain
- Issues that require physical access to a user's machine
- Social engineering of the core team

## Deployment Status

The program is **pre-deployment** as of the most recent release. No mainnet
contract address is published until deployment is announced through the
official channels.

Treat any "mainnet contract address" not published here or in
`/.well-known/shain.json` on the official domain as unauthorized.

## Bounty

A bounty program will be announced once the program is deployed to mainnet
and an independent audit has completed. In the interim, responsible
disclosure is welcomed and credited.
