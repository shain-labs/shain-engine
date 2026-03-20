.PHONY: help build test lint format clean deploy-devnet idl sdk cli

help:
	@echo "Shain — make targets"
	@echo "  build            build the on-chain program (anchor build)"
	@echo "  test             run the litesvm integration test suite"
	@echo "  lint             run cargo clippy across the workspace"
	@echo "  format           run cargo fmt --all"
	@echo "  format-check     check formatting without modifying files"
	@echo "  clean            remove build artifacts"
	@echo "  idl              export the Anchor IDL JSON"
	@echo "  sdk              build the TypeScript SDK"
	@echo "  cli              build the Rust CLI"
	@echo "  deploy-devnet    deploy the program to Solana devnet"

build:
	anchor build

test:
	cargo test --package shain --test test_shain

lint:
	cargo clippy --workspace --all-targets -- -W clippy::all

format:
	cargo fmt --all

format-check:
	cargo fmt --all -- --check

clean:
	cargo clean
	rm -rf .anchor target sdk/dist sdk/node_modules

idl:
	anchor idl build --program-name shain -o idl/shain.json

sdk:
	cd sdk && npm install && npm run build

cli:
	cargo build --release -p shain-cli

deploy-devnet:
	anchor deploy --provider.cluster devnet
