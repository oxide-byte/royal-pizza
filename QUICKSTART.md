# Royal Pizza - Quick Start Guide

## 🚀 Development (Recommended for local work)

```bash
# 1. Start database only
docker-compose -f docker-compose.dev.yml up -d

# 2. Start backend (Terminal 1)
cd backend
cargo run

# 3. Start frontend (Terminal 2)
cd frontend
trunk serve

# 4. Access the application
# Frontend: http://localhost:8080
# Backend API: http://localhost:8080/api
# Database: localhost:8000
```

**Stop development environment:**
```bash
docker-compose -f docker-compose.dev.yml down
```

---

## 🏭 Production (Full stack in Docker)

```bash
# 1. Generate secure JWT secret
./scripts/generate-jwt-secret.sh

# 2. Update .env.production with the generated secret
# Edit: .env.production
# Set: JWT_SECRET=<generated-secret>

# 3. Build and start all services
docker-compose up -d --build

# 4. Verify services are healthy
docker-compose ps

# 5. Test the API
curl http://localhost:8080/api/health
```

**Stop production environment:**
```bash
docker-compose down
```

**View logs:**
```bash
docker-compose logs -f
```

---

## 📋 Environment Setup

### First Time Setup

1. **Copy environment template:**
   ```bash
   cp .env.example .env.development
   ```

2. **For production, generate secrets:**
   ```bash
   ./scripts/generate-jwt-secret.sh
   cp .env.production.example .env.production
   # Edit .env.production with generated secrets
   ```

### Required Environment Variables

**Development (.env.development):**
- `DATABASE_URL=localhost:8000` ✓
- `JWT_SECRET=dev-secret-not-for-production` ✓
- `RUST_LOG=debug` ✓

**Production (.env.production):**
- `DATABASE_URL=ws://surrealdb:8000/rpc` ✓
- `JWT_SECRET=<generated-secure-secret>` ⚠️ REQUIRED
- `DATABASE_PASSWORD=<secure-password>` ⚠️ REQUIRED
- `CORS_ALLOW_ORIGIN=https://your-domain.com` ⚠️ UPDATE
- `RUST_LOG=info` ✓

---

## 🔍 Health Checks

**Backend Health:**
```bash
curl http://localhost:8080/api/health
```

**Expected Response:**
```json
{
  "status": "healthy",
  "service": "royal-pizza-backend",
  "version": "0.1.0",
  "services": {
    "database": "connected"
  },
  "uptime_seconds": 120,
  "timestamp": "2026-02-13T10:00:00Z"
}
```

---

## 🛠️ Common Commands

### Development

```bash
# Run backend with auto-reload (if cargo-watch installed)
cargo watch -x run

# Run frontend with hot-reload
trunk serve --open

# Check code
cargo check

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Production

```bash
# Build release binary
cargo build --release

# Start production stack
docker-compose up -d

# View service status
docker-compose ps

# View logs
docker-compose logs -f backend
docker-compose logs -f frontend
docker-compose logs -f surrealdb

# Restart a service
docker-compose restart backend

# Stop all services
docker-compose down

# Stop and remove volumes (⚠️ deletes data)
docker-compose down -v
```

---

## 📚 Documentation

- **Full Docker Guide**: See [DOCKER.md](DOCKER.md)
- **Phase 8 Summary**: See [PHASE8_SUMMARY.md](PHASE8_SUMMARY.md)
- **Storyboard**: See [storyboard.md](storyboard.md)
- **Architecture**: See [architecture.md](architecture.md) (update in Phase 14)

---

## 🐛 Troubleshooting

### Backend won't start

```bash
# Check database is running
docker-compose -f docker-compose.dev.yml ps

# Check database connectivity
curl http://localhost:8000/health

# Check backend logs
cargo run  # Look for error messages
```

### Frontend won't start

```bash
# Check Trunk is installed
trunk --version

# Install Trunk if needed
cargo install trunk

# Check wasm target is installed
rustup target add wasm32-unknown-unknown
```

### Port already in use

```bash
# Find process using port 8080
lsof -i :8080

# Kill the process
kill -9 <PID>
```

### Docker issues

```bash
# Clean up Docker
docker-compose down -v
docker system prune -a

# Rebuild from scratch
docker-compose build --no-cache
docker-compose up -d
```

---

## ✅ Verification Checklist

### Development Environment
- [ ] Database starts: `docker-compose -f docker-compose.dev.yml up -d`
- [ ] Backend compiles: `cargo check`
- [ ] Backend runs: `cd backend && cargo run`
- [ ] Frontend compiles: `cd frontend && trunk build`
- [ ] Frontend serves: `cd frontend && trunk serve`
- [ ] Health endpoint works: `curl http://localhost:8080/api/health`

### Production Environment
- [ ] JWT secret generated
- [ ] `.env.production` configured
- [ ] Stack builds: `docker-compose build`
- [ ] Stack starts: `docker-compose up -d`
- [ ] All services healthy: `docker-compose ps`
- [ ] Health check passes: `curl http://localhost:8080/api/health`
- [ ] Data persists after restart

---