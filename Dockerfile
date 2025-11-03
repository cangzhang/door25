FROM rust:1-slim AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        build-essential \
        pkg-config \
        libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/

COPY . .

RUN cargo build --release

FROM debian:trixie-slim

WORKDIR /usr/app

COPY --from=builder /usr/src/assets assets
COPY --from=builder /usr/src/config config
COPY --from=builder /usr/src/target/release/door25-cli door25-cli

ENTRYPOINT ["/usr/app/door25-cli"]
