# Stage 1: Build WASM application
FROM rust:1.93-bookworm as builder

# Install Trunk and wasm target
RUN cargo install trunk && \
    rustup target add wasm32-unknown-unknown

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY frontend ./frontend
COPY shared ./shared
COPY backend ./backend

# Build frontend with Trunk
WORKDIR /app/frontend

# Set API base URL for container networking
ARG API_BASE_URL=http://backend:8080/api
ENV TRUNK_API_BASE_URL=${API_BASE_URL}

RUN trunk build --release

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
