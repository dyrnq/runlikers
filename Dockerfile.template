# syntax=docker/dockerfile:1
FROM rust:1.85-slim-bookworm AS chef
RUN apt-get update && apt-get install -y --no-install-recommends musl-tools && rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-chef --locked
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM alpine:3.21
RUN apk add --no-cache ca-certificates docker-cli
COPY --from=builder /app/target/release/runlikers /usr/local/bin/runlikers
ENTRYPOINT ["runlikers"]
CMD ["--help"]
