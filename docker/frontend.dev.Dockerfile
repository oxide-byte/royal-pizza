FROM rust:1.93-bookworm

# Install Trunk and wasm target
RUN cargo install trunk && \
    rustup target add wasm32-unknown-unknown

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY frontend ./frontend
COPY shared ./shared
COPY backend ./backend

WORKDIR /app/frontend

EXPOSE 3000

# Use Trunk dev server for hot reload
CMD ["trunk", "serve", "--address", "0.0.0.0", "--port", "3000"]
