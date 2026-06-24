FROM rust:1-slim AS builder
WORKDIR /app
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY src src
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/rust-gateway /usr/local/bin/rust-gateway
EXPOSE 8000
CMD ["rust-gateway"]
