FROM rust:1.93-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.toml
COPY src src

RUN cargo build --release

FROM debian:bookworm-slim

RUN useradd --create-home --shell /usr/sbin/nologin appuser

WORKDIR /app

COPY --from=builder /app/target/release/re-indicators-calculation-service /usr/local/bin/re-indicators-calculation-service

EXPOSE 8081

USER appuser

CMD ["re-indicators-calculation-service"]
