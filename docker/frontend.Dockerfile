# Stage 1: Build WASM application
FROM rust:1.93-bookworm as builder

# Install Trunk and wasm-bindgen-cli (required for WASM builds)
RUN cargo install trunk wasm-bindgen-cli && \
    rustup target add wasm32-unknown-unknown

WORKDIR /app

# Copy workspace configuration first
COPY Cargo.toml Cargo.lock ./

# Copy only the crates needed for frontend build
COPY shared ./shared
COPY frontend ./frontend

# Dummy backend to satisfy workspace (not actually built)
RUN mkdir -p backend/src && \
    echo 'fn main() {}' > backend/src/main.rs && \
    echo '[package]\nname = "backend"\nversion = "0.1.0"\nedition = "2024"\n\n[dependencies]' > backend/Cargo.toml

# Build frontend with Trunk
WORKDIR /app/frontend

# Set API base URL for container networking
ARG API_BASE_URL=http://backend:8080/api
ENV TRUNK_API_BASE_URL=${API_BASE_URL}

# Use Docker-specific config with wasm-opt disabled
# This avoids memory issues in Docker builds while still using release mode
RUN cp Trunk.docker.toml Trunk.toml && trunk build --release

# Stage 2: Serve with nginx
FROM nginx:1.27-alpine

# Copy built static files
COPY --from=builder /app/frontend/dist /usr/share/nginx/html

# Copy custom nginx configuration
COPY docker/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=3s --retries=3 \
  CMD wget --quiet --tries=1 --spider http://localhost:3000/health || exit 1

CMD ["nginx", "-g", "daemon off;"]
