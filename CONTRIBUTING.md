# Contributing to Shain

Thanks for your interest in Shain. This document describes how to get a
development environment up and how we expect changes to flow into `main`.

## Ground rules

- **Read the source, not the website.** The site is marketing; the repo is
  canon. All behaviour claims you file against must be backed by a failing
  test in `programs/shain/tests/` or `tests/integration/`.
- **One change per pull request.** If a PR description starts drifting into
  bullet points, split it.
- **No behaviour change without tests.** Every fix and every feature ships
  with a regression test.
- **Conventional commits.** Your branch history does not need to be clean,
  but the squash commit that lands on `main` does.

## Development environment

```bash
git clone https://github.com/shainprotocol/shain-engine
cd shain-engine
rustup toolchain install 1.89.0
solana-install init 3.1.13
avm install 1.0.0 && avm use 1.0.0
anchor build
cargo test --package shain --test test_shain
```

The first `anchor build` will take several minutes while the Anchor and
Solana toolchains cache. Subsequent builds are incremental.

## Branching

- `main` is always deployable to devnet.
- Feature branches: `feat/<short-slug>` — merged by squash.
- Fix branches: `fix/<short-slug>` — merged by squash.
- Long-running refactors: open an issue first.

## Commit messages

We use conventional commits. Valid prefixes:

| prefix      | use when                                                |
| ----------- | ------------------------------------------------------- |
| `feat`      | new user-facing behaviour                               |
| `fix`       | bug fix                                                 |
| `refactor`  | behaviour unchanged, structure improved                 |
| `perf`      | measurable performance improvement                      |
| `docs`      | documentation only                                      |
| `test`      | new or updated tests                                    |
| `chore`     | tooling, housekeeping, non-code changes                 |
| `ci`        | GitHub Actions, release automation                      |
| `deps`      | dependency bumps                                        |
| `style`     | formatting only                                         |

## Pull request checklist

Before opening a PR, make sure:

- [ ] `cargo test --package shain --test test_shain` passes locally
- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo clippy --workspace -- -W warnings` is green
- [ ] You added a line to `CHANGELOG.md` under `Unreleased`
- [ ] The PR description references an issue or explains the motivation

## Security

Do not open public issues for security problems. See [SECURITY.md](./SECURITY.md)
for the reporting process.

## Code of conduct

By participating you agree to abide by our [Code of Conduct](./CODE_OF_CONDUCT.md).
