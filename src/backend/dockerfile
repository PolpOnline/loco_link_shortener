FROM rust as builder

WORKDIR /usr/src/

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim as production

RUN apt-get update && apt install -y openssl ca-certificates

WORKDIR /usr/app

COPY --from=builder /usr/src/frontend/build /usr/app/frontend/build
COPY --from=builder /usr/src/frontend/build/index.html /usr/app/frontend/build/index.html
COPY --from=builder /usr/src/config /usr/app/config
COPY --from=builder /usr/src/target/release/loco_link_shortener-cli /usr/app/loco_link_shortener-cli

EXPOSE 443

ENTRYPOINT ["/usr/app/loco_link_shortener-cli"]
CMD ["start", "-e", "production"]