# Docker Configuration Guide

## Overview

Royal Pizza uses separate Docker configurations for development and production environments:

- **Production**: `docker-compose.yml` - Full stack deployment
- **Development**: `docker-compose.dev.yml` - Database only, run backend/frontend from IDE

---

## Development Workflow

For local development with hot-reload and full IDE support:

### 1. Start Database Only

```bash
docker-compose -f docker-compose.dev.yml up -d
```

This starts only the SurrealDB database in a Docker container.

### 2. Start Backend (Terminal 1)

```bash
cd backend
cargo run
```

The backend will run at `http://localhost:8080` with hot-reload via `cargo watch` (if installed).

### 3. Start Frontend (Terminal 2)

```bash
cd frontend
trunk serve
```

The frontend will run at `http://localhost:8080` (Trunk's default port) with hot-reload.

### Benefits of Development Mode

- ✅ Fast hot-reload on code changes
- ✅ Full IDE debugging support
- ✅ No Docker rebuilds required
- ✅ Native Rust tooling (cargo check, clippy, fmt)
- ✅ Direct access to logs
- ✅ Easy to switch between debug/release builds

### Stop Development Environment

```bash
docker-compose -f docker-compose.dev.yml down
```

---

## Production Workflow

For production deployment with optimized builds:

### 1. Configure Environment Variables

Copy and edit the production environment file:

```bash
cp .env.production.example .env.production
```

**Critical**: Generate a secure JWT secret:

```bash
./scripts/generate-jwt-secret.sh
```

Update `.env.production` with:
- Secure `JWT_SECRET`
- Strong `DATABASE_PASSWORD`
- Production `CORS_ALLOW_ORIGIN` (your domain)
- `ADMIN_PASSWORD` for Ferriskey

### 2. Build and Start All Services

```bash
docker-compose up -d --build
```

This will:
- Build optimized backend binary with LTO and strip symbols
- Build frontend WASM with release optimizations
- Start SurrealDB with persistent storage
- Set up networking between services
- Configure health checks

### 3. Verify Services

Check service health:

```bash
docker-compose ps
```

All services should show "healthy" status.

Test the API:

```bash
curl http://localhost:8080/api/health
```

Expected response:
```json
{
  "status": "healthy",
  "service": "royal-pizza-backend",
  "database": "connected",
  "timestamp": "2026-02-13T10:00:00Z"
}
```

### 4. View Logs

```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f backend
docker-compose logs -f frontend
docker-compose logs -f surrealdb
```

### 5. Stop Production Environment

```bash
docker-compose down
```

To stop and remove volumes (⚠️ **this deletes all data**):

```bash
docker-compose down -v
```

---

## Service Architecture

### Production Stack

```
┌─────────────────────────────────────┐
│  Frontend (Nginx + WASM)            │
│  Port: 3000                         │
└────────────┬────────────────────────┘
             │
             ↓
┌─────────────────────────────────────┐
│  Backend (Axum API)                 │
│  Port: 8080                         │
└────────────┬────────────────────────┘
             │
             ↓
┌─────────────────────────────────────┐
│  SurrealDB (Database)               │
│  Port: 8000                         │
│  Volume: surrealdb_prod_data        │
└─────────────────────────────────────┘
```

### Development Stack

```
┌─────────────────────────────────────┐
│  Frontend (Trunk - Local)           │
│  http://localhost:8080              │
└────────────┬────────────────────────┘
             │
             ↓
┌─────────────────────────────────────┐
│  Backend (Cargo - Local)            │
│  http://localhost:8080/api          │
└────────────┬────────────────────────┘
             │
             ↓
┌─────────────────────────────────────┐
│  SurrealDB (Docker)                 │
│  Port: 8000                         │
└─────────────────────────────────────┘
```

---

## Health Checks

All production services have health checks configured:

### Backend Health Check
- **Endpoint**: `http://localhost:8080/api/health`
- **Interval**: 30 seconds
- **Timeout**: 10 seconds
- **Retries**: 3
- **Checks**: Service availability, database connectivity

### Frontend Health Check
- **Method**: HTTP GET to root
- **Interval**: 30 seconds
- **Timeout**: 10 seconds
- **Retries**: 3

### Database Health Check
- **Endpoint**: `http://localhost:8000/health`
- **Interval**: 30 seconds (production), 10 seconds (development)
- **Timeout**: 10 seconds
- **Retries**: 3

---

## Volumes and Data Persistence

### Production Volume
- **Name**: `surrealdb_prod_data`
- **Location**: Docker managed volume
- **Persistence**: Data survives container restarts

### Backup Database

```bash
docker run --rm --volumes-from royalpizza_db_prod -v $(pwd):/backup \
  ubuntu tar cvf /backup/backup.tar /data
```

### Restore Database

```bash
docker run --rm --volumes-from royalpizza_db_prod -v $(pwd):/backup \
  ubuntu tar xvf /backup/backup.tar
```

---

## Environment Variables

### Required Variables (Production)

| Variable | Description | Example |
|----------|-------------|---------|
| `DATABASE_URL` | SurrealDB connection URL | `ws://surrealdb:8000/rpc` |
| `DATABASE_PASSWORD` | Database root password | Strong random password |
| `JWT_SECRET` | JWT signing secret | Generated with script |
| `FERRISKEY_URL` | Authentication service URL | `http://ferriskey:8081` |
| `CORS_ALLOW_ORIGIN` | Allowed CORS origins | `https://yourdomain.com` |

### Optional Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `PORT` | Backend port | `8080` |
| `HOST` | Backend host | `0.0.0.0` |
| `RUST_LOG` | Log level | `info` |
| `DATABASE_SEED` | Seed database on startup | `false` |

---

## Build Optimizations

### Rust Release Profile

The workspace `Cargo.toml` includes aggressive optimizations:

```toml
[profile.release]
opt-level = 3              # Maximum optimization
lto = true                 # Link-time optimization
codegen-units = 1          # Single compilation unit
strip = true               # Strip debug symbols
panic = "abort"            # Smaller binary
```

### Expected Binary Sizes
- **Backend**: ~15-20 MB (stripped)
- **Frontend WASM**: ~1-2 MB (optimized)

### Build Times
- **Backend**: 5-10 minutes (first build), 2-3 minutes (incremental)
- **Frontend**: 3-5 minutes (first build), 1-2 minutes (incremental)

---

## Troubleshooting

### Backend Won't Start

1. Check database connectivity:
   ```bash
   docker-compose logs surrealdb
   ```

2. Verify environment variables:
   ```bash
   docker-compose config
   ```

3. Check backend logs:
   ```bash
   docker-compose logs backend
   ```

### Frontend Not Accessible

1. Check if nginx is running:
   ```bash
   docker-compose ps frontend
   ```

2. Check nginx logs:
   ```bash
   docker-compose logs frontend
   ```

3. Verify build artifacts:
   ```bash
   docker-compose exec frontend ls -la /usr/share/nginx/html
   ```

### Database Connection Issues

1. Verify database is healthy:
   ```bash
   curl http://localhost:8000/health
   ```

2. Check network connectivity:
   ```bash
   docker-compose exec backend ping surrealdb
   ```

3. Verify credentials in `.env.production`

### Port Conflicts

If ports 3000, 8000, or 8080 are in use:

1. Stop conflicting services:
   ```bash
   lsof -i :8080
   kill <PID>
   ```

2. Or change ports in `docker-compose.yml`:
   ```yaml
   ports:
     - "8081:8080"  # Map to different host port
   ```

---

## Security Considerations

### Production Checklist

- [ ] Generate strong JWT secret with `./scripts/generate-jwt-secret.sh`
- [ ] Set strong database password
- [ ] Configure CORS for production domain only
- [ ] Use HTTPS in production (add reverse proxy)
- [ ] Don't commit `.env.production` to version control
- [ ] Regularly update Docker images
- [ ] Monitor logs for security issues
- [ ] Set up database backups
- [ ] Use Docker secrets for sensitive data (advanced)

### HTTPS Setup (Recommended)

Add a reverse proxy (nginx or Caddy) in front of the stack:

```yaml
services:
  nginx-proxy:
    image: nginx:alpine
    ports:
      - "443:443"
      - "80:80"
    volumes:
      - ./nginx-proxy.conf:/etc/nginx/nginx.conf
      - /etc/letsencrypt:/etc/letsencrypt
```

---

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Deploy

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Build and deploy
        run: |
          docker-compose build
          docker-compose up -d
          
      - name: Health check
        run: |
          sleep 30
          curl -f http://localhost:8080/api/health
```

---

## Performance Tuning

### Database Optimization

For production, consider using file-based storage:

```yaml
command: start --log info --user root --pass ${DB_PASSWORD} \
  --bind 0.0.0.0:8000 file:/data/database.db
```

### Backend Tuning

Adjust Tokio runtime in `backend/src/main.rs`:

```rust
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // ...
}
```

### Frontend Optimization

Enable compression in nginx configuration:

```nginx
gzip on;
gzip_types text/css application/javascript application/wasm;
```

---

## Next Steps

After completing Phase 8, proceed to:

- **Phase 9**: Code Quality & Refactoring
- **Phase 10**: E2E Testing with Playwright
- **Phase 11**: IAM Integration with Ferriskey

See `storyboard.md` for detailed implementation plans.
