FROM lukemathwalker/cargo-chef:latest-rust-1.90.0 AS chef
WORKDIR /app
RUN apt-get update && apt-get install lld clang curl ca-certificates -y

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build stage
FROM chef AS builder
COPY  --from=planner /app/recipe.json recipe.json

RUN curl -Ls https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz | tar -xz && \
    mv cargo-binstall /usr/local/cargo/bin/
# this Tailwind version works fine on railway server, for arm use tailwindcss-linux-arm64
RUN curl -Ls https://github.com/tailwindlabs/tailwindcss/releases/download/v4.1.17/tailwindcss-linux-x64 -o tailwindcss && \
    mv tailwindcss /usr/local/

RUN chmod +x /usr/local/tailwindcss

ENV SINGLESTAGE_TAILWIND_PATH=/usr/local/tailwindcss

RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE=true
ENV LEPTOS_WASM_BINDGEN_VERSION=0.2.105
RUN rustup target add wasm32-unknown-unknown
RUN cargo binstall cargo-leptos -y
RUN cargo leptos build --release


# Runtime stage
FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/leptos-question-bank /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/

COPY config config
ENV APP_ENVIRONMENT=production

# Set any required env variables and
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 3000

ENTRYPOINT ["/app/leptos-question-bank"]
