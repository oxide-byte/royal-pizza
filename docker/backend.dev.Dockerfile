FROM rust:1.93-bookworm
WORKDIR /app

# Install development dependencies
RUN apt-get update && \
    apt-get install -y libssl-dev pkg-config && \
    cargo install cargo-watch && \
    rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY backend ./backend
COPY shared ./shared
COPY frontend ./frontend
COPY database ./database

# Build in debug mode for faster compilation
RUN cargo build --bin backend

EXPOSE 8080

# Use cargo-watch for hot reload
CMD ["cargo", "watch", "-x", "run --bin backend"]
