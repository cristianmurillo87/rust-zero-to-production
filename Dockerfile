FROM lukemathwalker/cargo-chef:latest-rust-1.84.1 AS chef
WORKDIR /app
RUN apt update && apt install lld clang -y

#caching
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# build stage
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin rust_zero_to_production

# run stage
FROM rust:1.84.1-slim-bullseye AS runtime
WORKDIR /app

RUN apt-get update -y \
    && apt-get upgrade -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust_zero_to_production rust_zero_to_production
COPY config config
ENV APP_ENVIRONMENT=production

ENTRYPOINT [ "./rust_zero_to_production" ]