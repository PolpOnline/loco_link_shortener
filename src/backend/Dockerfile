FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .

RUN apt-get update && apt install -y mold clang

ENV RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=/usr/bin/mold"

RUN cargo build --release --bin loco_link_shortener-cli

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt install -y openssl ca-certificates

WORKDIR /app
COPY --from=builder /app/config /app/config
COPY --from=builder /app/target/release/loco_link_shortener-cli /usr/local/bin

EXPOSE 443
ENTRYPOINT ["/usr/local/bin/loco_link_shortener-cli"]
CMD ["start", "-e", "production"]