# Stage 1: Dependency caching layer
FROM rust:1.93-bookworm as chef
RUN cargo install cargo-chef
WORKDIR /app

# Stage 2: Analyze dependencies
FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml ./backend/
COPY shared/Cargo.toml ./shared/
COPY frontend/Cargo.toml ./frontend/
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: Build dependencies (cached layer)
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Stage 4: Build application
COPY backend ./backend
COPY shared ./shared
COPY frontend ./frontend
RUN cargo build --release --bin backend

# Stage 5: Runtime image
FROM debian:bookworm-slim
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 curl && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/backend /usr/local/bin/backend

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=3s --retries=3 \
  CMD curl -f http://localhost:8080/api/health || exit 1

CMD ["backend"]
