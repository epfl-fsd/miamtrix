FROM rust:1.93-slim-bookworm AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -R src/

COPY migrations ./migrations

COPY src ./src

RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libsqlite3-0 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

RUN useradd -ms /bin/false miamtrix

COPY --from=builder /usr/src/app/target/release/miamtrix ./miamtrix

RUN chown miamtrix:miamtrix ./miamtrix

USER miamtrix

CMD ["./miamtrix"]
