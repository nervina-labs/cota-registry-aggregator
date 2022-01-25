FROM clux/muslrust:stable as builder

WORKDIR /app

COPY . .
COPY debian/config .cargo/config.toml

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release

FROM alpine:latest
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/cota-registry-aggregator /app/cota-registry-aggregator
RUN chmod +x /app/cota-registry-aggregator

WORKDIR /app

ENV RUST_LOG info
ENV DATABASE_URL mysql://root:password@localhost:3306/db_name
ENV MAX_POOL 20
ENV MIN_POOL 10

EXPOSE 3050

CMD ["./cota-registry-aggregator"]
