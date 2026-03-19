FROM rust:1.89-slim-bookworm AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev build-essential ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml rust-toolchain.toml ./
COPY programs/shain/Cargo.toml programs/shain/

RUN mkdir -p programs/shain/src \
    && echo "fn main() {}" > programs/shain/src/lib.rs \
    && cargo fetch || true

COPY programs programs

RUN cargo build --release --package shain || cargo check --release --package shain

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -r -u 1001 -m shain

COPY --from=builder /app/target/release /usr/local/lib/shain
COPY --from=builder /app/Cargo.toml /usr/local/lib/shain/Cargo.toml

USER shain
WORKDIR /home/shain

ENTRYPOINT ["/bin/sh"]
