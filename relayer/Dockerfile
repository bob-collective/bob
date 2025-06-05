FROM rust:1-slim AS builder

WORKDIR /usr/src/app

COPY . .

RUN apt-get update \
    && apt-get install -y libssl-dev pkg-config
RUN cargo build --release \
    -p bob-relayer

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y openssl ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/bob-relayer /usr/local/bin/bob-relayer

ENTRYPOINT ["/usr/local/bin/bob-relayer"]
