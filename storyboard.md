# Royal Pizza - Development Storyboard (Sprint 2)

## Executive Summary

**Project**: Royal Pizza - Production Hardening & Authentication
**Timeline**: Sprint 2 - 8 days (64 hours)
**Tech Stack**: Rust + Leptos + Axum + SurrealDB + Ferriskey IAM + Playwright
**Delivery**: Production-ready system with authentication, E2E testing, and order history

---

## Project Phases Overview (Sprint 2)

| Phase | Duration | Description |
|-------|----------|-------------|
| **Phase 8: Production Docker** | 4 hours | Clean docker-compose for production, separate dev/prod environments |
| **Phase 9: Code Quality** | 6 hours | Review, refactor, clean Clippy warnings, optimize code |
| **Phase 10: E2E Testing** | 10 hours | Playwright test suite, CI/CD integration, test automation |
| **Phase 11: IAM Integration** | 12 hours | Integrate Ferriskey, JWT authentication, user management |
| **Phase 12: Authenticated Orders** | 8 hours | Require authentication for orders, user-specific flows |
| **Phase 13: Order History** | 10 hours | Store and retrieve order history, user dashboard |
| **Phase 14: Documentation** | 4 hours | Update architecture.md, business.md, API docs |
| **Phase 15: Final Testing** | 10 hours | Security audit, performance testing, production deployment |

**Total Estimated Time**: 64 hours

---

## Phase 8: Production Docker Configuration (4 hours)

### Task 8.1: Separate Development Environment
**Estimation**: 1.5 hours
**Priority**: Critical
**Depends On**: Sprint 1 Phase 6

**Deliverables**:
- Clean existing `docker-compose.yml` to represent ONLY production environment
- Create `docker-compose.dev.yml` for development with hot-reload support
- Production docker-compose contains: database, backend, frontend (built)
- Development workflow: Run database in Docker, start backend/frontend from IDE
- Document both workflows in README

**Production docker-compose.yml**:
```yaml
version: '3.8'
services:
  database:
    image: surrealdb/surrealdb:2.6.0
    environment:
      - SURREAL_USER=root
      - SURREAL_PASS=${DB_ROOT_PASSWORD}
    volumes:
      - pizza_data:/data
    ports:
      - "8000:8000"
    command: start --log trace file:/data/database.db

  backend:
    build:
      context: .
      dockerfile: backend/Dockerfile
    environment:
      - DATABASE_URL=ws://database:8000
      - RUST_LOG=info
      - JWT_SECRET=${JWT_SECRET}
    ports:
      - "8080:8080"
    depends_on:
      - database

  frontend:
    build:
      context: .
      dockerfile: frontend/Dockerfile
    ports:
      - "3000:3000"
    depends_on:
      - backend

volumes:
  pizza_data:
```

**Development Workflow**:
- `docker-compose -f docker-compose.dev.yml up database` (only DB)
- Run backend: `cd backend && cargo run`
- Run frontend: `cd frontend && trunk serve`

**Success Criteria**:
- Production compose deploys full stack
- Development compose runs only database
- Clear documentation for both workflows

---

### Task 8.2: Environment Variable Management
**Estimation**: 1 hour
**Priority**: High
**Depends On**: Task 8.1

**Deliverables**:
- Create `.env.production.example` template
- Update `.env.development` for local IDE development
- Add JWT secret generation script
- Document all required environment variables
- Add validation for missing environment variables in backend startup

**Environment Variables**:

**Production**:
```bash
# Database
DB_ROOT_PASSWORD=<secure-password>
DATABASE_URL=ws://database:8000
DATABASE_NAMESPACE=royalpizza
DATABASE_NAME=production

# Backend
JWT_SECRET=<generated-secret>
FERRISKEY_URL=http://ferriskey:8081
RUST_LOG=info
PORT=8080

# CORS
CORS_ALLOW_ORIGIN=http://localhost:3000,https://your-domain.com
```

**Development**:
```bash
DATABASE_URL=localhost:8000
DATABASE_NAMESPACE=royalpizza
DATABASE_NAME=development
JWT_SECRET=dev-secret-change-in-production
FERRISKEY_URL=http://localhost:8081
RUST_LOG=debug
PORT=8080
CORS_ALLOW_ORIGIN=http://localhost:3000
```

**Files**:
- `.env.production.example`
- `.env.development`
- `scripts/generate-jwt-secret.sh`

---

### Task 8.3: Health Checks & Monitoring
**Estimation**: 1 hour
**Priority**: Medium
**Depends On**: Task 8.1

**Deliverables**:
- Add Docker health checks to all services
- Implement readiness probe endpoints
- Add service dependency health monitoring
- Configure restart policies
- Add logging configuration for production

**Health Check Implementation**:
```dockerfile
# backend/Dockerfile
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8080/api/health || exit 1
```

**Backend Health Endpoint Enhancement**:
```rust
// Check database, Ferriskey, and service health
{
  "status": "healthy",
  "timestamp": "2026-02-13T10:00:00Z",
  "services": {
    "database": "connected",
    "ferriskey": "reachable",
    "uptime_seconds": 3600
  }
}
```

**File**: Update `backend/src/handlers/health.rs`

---

### Task 8.4: Production Optimization
**Estimation**: 0.5 hours
**Priority**: Medium
**Depends On**: Task 8.3

**Deliverables**:
- Configure Rust release profile for optimal binary size
- Enable LTO (Link Time Optimization)
- Strip debug symbols
- Optimize Docker layer caching
- Add .dockerignore file

**Cargo.toml optimization**:
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
panic = "abort"
```

**.dockerignore**:
```
target/
.git/
.env*
*.md
.vscode/
.idea/
node_modules/
dist/
```

**Success Criteria**:
- Backend binary size < 20MB
- Frontend WASM size < 2MB
- Docker images use layer caching effectively

---

## Phase 9: Code Quality & Refactoring (6 hours)

### Task 9.1: Clippy Warnings Resolution
**Estimation**: 2 hours
**Priority**: High
**Depends On**: Sprint 1 Phase 7

**Deliverables**:
- Run `cargo clippy --all-targets --all-features` across workspace
- Fix all warnings and errors
- Enable clippy in CI pipeline
- Add `clippy.toml` with project-specific rules
- Document any intentionally allowed warnings

**Clippy Configuration** (`clippy.toml`):
```toml
# Deny clippy warnings in CI
# cargo clippy -- -D warnings

# Custom rules
too-many-arguments-threshold = 5
type-complexity-threshold = 250
```

**Common Issues to Fix**:
- Unnecessary clones
- Redundant field names in struct initialization
- Needless borrows
- Unused imports
- Inefficient string concatenation
- Missing `#[must_use]` attributes

**Files**: All Rust files in workspace

---

### Task 9.2: Code Review & Refactoring
**Estimation**: 3 hours
**Priority**: High
**Depends On**: Task 9.1

**Deliverables**:
- Review all modules for code duplication
- Extract common patterns into utilities
- Improve error messages for better debugging
- Add missing documentation comments
- Refactor long functions (> 50 lines)
- Improve type safety with newtype patterns
- Remove dead code

**Refactoring Targets**:
1. **Error Handling**: Consolidate error types across modules
2. **Validation Logic**: Move to shared crate for reuse
3. **Repository Layer**: Add generic repository trait
4. **API Client**: Extract common request/response handling
5. **Constants**: Move magic numbers to configuration

**Example Refactoring**:
```rust
// Before: Magic numbers
if pickup_time < now + Duration::minutes(30) { ... }

// After: Named constants
const MIN_PICKUP_LEAD_TIME_MINUTES: i64 = 30;
if pickup_time < now + Duration::minutes(MIN_PICKUP_LEAD_TIME_MINUTES) { ... }
```

**Files**: All Rust files, focus on:
- `backend/src/handlers/`
- `backend/src/services/`
- `shared/src/validation/`

---

### Task 9.3: Performance Optimization
**Estimation**: 1 hour
**Priority**: Medium
**Depends On**: Task 9.2

**Deliverables**:
- Profile backend API endpoints
- Optimize database queries (add indexes if needed)
- Reduce unnecessary allocations
- Cache frequently accessed data (pizza menu)
- Add connection pooling tuning
- Benchmark critical paths

**Optimization Areas**:
1. **Database Queries**: Add prepared statements, batch operations
2. **Serialization**: Use zero-copy deserialization where possible
3. **Memory**: Reduce cloning, use references
4. **Caching**: Add in-memory cache for pizza menu (updates rarely)

**Performance Targets**:
- API response time p95 < 100ms
- Menu load time < 50ms
- Order creation < 200ms
- Database query time < 20ms

**Tools**:
- `cargo flamegraph` for profiling
- `hyperfine` for benchmarking
- SurrealDB query analyzer

---

## Phase 10: E2E Testing with Playwright (10 hours)

### Task 10.1: Playwright Setup
**Estimation**: 2 hours
**Priority**: Critical
**Depends On**: Phase 9

**Deliverables**:
- Initialize Playwright project in `e2e/` directory
- Install Playwright with browsers (Chromium, Firefox, WebKit)
- Configure TypeScript for tests
- Set up test environment configuration
- Create test utilities and helpers
- Configure CI/CD integration

**Project Structure**:
```
e2e/
├── playwright.config.ts
├── package.json
├── tests/
│   ├── auth.spec.ts
│   ├── menu.spec.ts
│   ├── ordering.spec.ts
│   ├── order-history.spec.ts
│   └── edge-cases.spec.ts
├── fixtures/
│   ├── users.json
│   └── test-data.ts
└── utils/
    ├── auth.ts
    ├── db-setup.ts
    └── helpers.ts
```

**playwright.config.ts**:
```typescript
export default defineConfig({
  testDir: './tests',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: 'html',
  use: {
    baseURL: 'http://localhost:3000',
    trace: 'on-first-retry',
  },
  projects: [
    { name: 'chromium', use: { ...devices['Desktop Chrome'] } },
    { name: 'firefox', use: { ...devices['Desktop Firefox'] } },
    { name: 'webkit', use: { ...devices['Desktop Safari'] } },
  ],
  webServer: {
    command: 'docker-compose up',
    url: 'http://localhost:3000',
    reuseExistingServer: !process.env.CI,
  },
});
```

---

### Task 10.2: Authentication Flow Tests
**Estimation**: 2 hours
**Priority**: Critical
**Depends On**: Task 10.1, Phase 11 (parallel)

**Deliverables**:
- Test user registration flow
- Test login with valid credentials
- Test login with invalid credentials
- Test logout functionality
- Test JWT token persistence
- Test token expiration handling
- Test protected route access

**Test Scenarios**:

**File**: `e2e/tests/auth.spec.ts`

```typescript
test.describe('Authentication', () => {
  test('should register new user', async ({ page }) => {
    await page.goto('/register');
    await page.fill('[name="email"]', 'test@example.com');
    await page.fill('[name="password"]', 'SecurePass123!');
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL('/menu');
  });

  test('should login existing user', async ({ page }) => {
    await page.goto('/login');
    await page.fill('[name="email"]', 'test@example.com');
    await page.fill('[name="password"]', 'SecurePass123!');
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL('/menu');
  });

  test('should reject invalid credentials', async ({ page }) => {
    await page.goto('/login');
    await page.fill('[name="email"]', 'test@example.com');
    await page.fill('[name="password"]', 'WrongPassword');
    await page.click('button[type="submit"]');
    await expect(page.locator('.error')).toContainText('Invalid credentials');
  });

  test('should redirect to login when accessing protected route', async ({ page }) => {
    await page.goto('/order-history');
    await expect(page).toHaveURL('/login');
  });
});
```

---

### Task 10.3: Ordering Flow Tests
**Estimation**: 3 hours
**Priority**: Critical
**Depends On**: Task 10.2

**Deliverables**:
- Test complete ordering flow (authenticated user)
- Test adding pizzas to cart
- Test cart operations (add, remove, update quantity)
- Test custom pizza ordering
- Test order form validation
- Test order submission
- Test order confirmation page

**Test Scenarios**:

**File**: `e2e/tests/ordering.spec.ts`

```typescript
test.describe('Ordering Flow', () => {
  test.beforeEach(async ({ page }) => {
    // Login before each test
    await loginUser(page, 'test@example.com', 'SecurePass123!');
  });

  test('should complete full ordering flow', async ({ page }) => {
    // Browse menu
    await page.goto('/menu');
    await expect(page.locator('.pizza-card')).toHaveCount(9);

    // Add pizza to cart
    await page.locator('.pizza-card').first().locator('button:has-text("Add to Cart")').click();
    await expect(page.locator('.cart-count')).toContainText('1');

    // Proceed to order
    await page.click('button:has-text("Proceed to Order")');
    await expect(page).toHaveURL('/order');

    // Fill order form
    await page.fill('[name="phone"]', '+1234567890');
    await page.fill('[name="pickup-date"]', getTomorrowDate());
    await page.selectOption('[name="pickup-time"]', '18:00');

    // Submit order
    await page.click('button:has-text("Place Order")');

    // Verify confirmation
    await expect(page).toHaveURL(/\/confirmation\/.*/, { timeout: 5000 });
    await expect(page.locator('.order-number')).toBeVisible();
  });

  test('should validate order form', async ({ page }) => {
    await page.goto('/menu');
    await page.locator('.pizza-card').first().locator('button:has-text("Add to Cart")').click();
    await page.click('button:has-text("Proceed to Order")');

    // Submit without phone
    await page.click('button:has-text("Place Order")');
    await expect(page.locator('.error')).toContainText('Phone is required');
  });

  test('should handle custom pizza', async ({ page }) => {
    await page.goto('/menu');

    // Add custom pizza
    await page.locator('.custom-pizza-card').locator('textarea').fill('Extra cheese, no onions');
    await page.locator('.custom-pizza-card').locator('select[name="size"]').selectOption('large');
    await page.locator('.custom-pizza-card').locator('button:has-text("Add to Cart")').click();

    await expect(page.locator('.cart-count')).toContainText('1');
  });
});
```

---

### Task 10.4: Order History Tests
**Estimation**: 1.5 hours
**Priority**: High
**Depends On**: Task 10.3

**Deliverables**:
- Test viewing order history
- Test filtering orders by status
- Test order details view
- Test empty order history state
- Test pagination (if implemented)

**File**: `e2e/tests/order-history.spec.ts`

---

### Task 10.5: Edge Cases & Error Scenarios
**Estimation**: 1.5 hours
**Priority**: High
**Depends On**: Task 10.4

**Deliverables**:
- Test network failures
- Test invalid order IDs
- Test expired JWT tokens
- Test concurrent cart operations
- Test browser back/forward navigation
- Test page refresh with state
- Test empty cart checkout attempt

**File**: `e2e/tests/edge-cases.spec.ts`

```typescript
test.describe('Edge Cases', () => {
  test('should handle network failure gracefully', async ({ page, context }) => {
    await loginUser(page, 'test@example.com', 'SecurePass123!');

    // Simulate offline
    await context.setOffline(true);
    await page.goto('/menu');

    await expect(page.locator('.error-message')).toContainText('Network error');

    // Restore connection
    await context.setOffline(false);
    await page.reload();
    await expect(page.locator('.pizza-card')).toHaveCount(9);
  });

  test('should handle invalid order ID', async ({ page }) => {
    await loginUser(page, 'test@example.com', 'SecurePass123!');
    await page.goto('/confirmation/invalid-id-123');

    await expect(page.locator('.error-message')).toContainText('Order not found');
  });

  test('should prevent checkout with empty cart', async ({ page }) => {
    await loginUser(page, 'test@example.com', 'SecurePass123!');
    await page.goto('/order');

    await expect(page.locator('button:has-text("Place Order")')).toBeDisabled();
  });
});
```

---

## Phase 11: IAM Integration with Ferriskey (12 hours)

### Task 11.1: Ferriskey Setup & Configuration
**Estimation**: 2 hours
**Priority**: Critical
**Depends On**: Phase 8

**Deliverables**:
- Clone and set up Ferriskey (https://github.com/ferriskey/ferriskey)
- Add Ferriskey to docker-compose
- Configure Ferriskey database and secrets
- Set up Ferriskey admin user
- Configure OAuth/OIDC settings
- Document Ferriskey setup process

**docker-compose.yml addition**:
```yaml
  ferriskey:
    image: ferriskey/ferriskey:latest
    environment:
      - DATABASE_URL=postgresql://ferriskey:password@ferriskey-db:5432/ferriskey
      - JWT_SECRET=${JWT_SECRET}
      - ADMIN_EMAIL=admin@royalpizza.com
      - ADMIN_PASSWORD=${ADMIN_PASSWORD}
    ports:
      - "8081:8081"
    depends_on:
      - ferriskey-db

  ferriskey-db:
    image: postgres:15
    environment:
      - POSTGRES_USER=ferriskey
      - POSTGRES_PASSWORD=password
      - POSTGRES_DB=ferriskey
    volumes:
      - ferriskey_data:/var/lib/postgresql/data

volumes:
  pizza_data:
  ferriskey_data:
```

**File**: Update `docker-compose.yml`

---

### Task 11.2: Backend JWT Authentication Middleware
**Estimation**: 3 hours
**Priority**: Critical
**Depends On**: Task 11.1

**Deliverables**:
- Create JWT validation middleware
- Integrate with Ferriskey for token verification
- Extract user ID from JWT claims
- Add authentication to protected routes
- Handle token expiration and refresh
- Add middleware to order endpoints

**Middleware Implementation**:

**File**: `backend/src/middleware/auth.rs`

```rust
use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // User ID
    pub email: String,
    pub exp: usize,         // Expiration
    pub iat: usize,         // Issued at
}

pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Verify JWT with Ferriskey's public key
    let decoding_key = DecodingKey::from_secret(
        std::env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set")
            .as_bytes()
    );

    let token_data = decode::<Claims>(
        token,
        &decoding_key,
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Add user info to request extensions
    req.extensions_mut().insert(token_data.claims);

    Ok(next.run(req).await)
}
```

**Apply to routes**:
```rust
// backend/src/routes/orders.rs
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/orders", post(create_order))
        .route("/api/orders/:id", get(get_order))
        .route("/api/orders/history", get(get_user_orders))
        .layer(middleware::from_fn(auth_middleware))
}
```

---

### Task 11.3: Frontend Authentication Service
**Estimation**: 3 hours
**Priority**: Critical
**Depends On**: Task 11.2

**Deliverables**:
- Create authentication API client
- Implement login/logout functions
- Store JWT in local storage or cookie
- Add authentication state management (Leptos signals)
- Create AuthContext for global auth state
- Add token refresh logic
- Handle authentication errors

**File**: `frontend/src/services/auth.rs`

```rust
use leptos::*;
use serde::{Deserialize, Serialize};
use gloo_storage::{LocalStorage, Storage};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthToken {
    pub access_token: String,
    pub expires_in: u64,
}

#[derive(Clone, Debug)]
pub struct AuthState {
    pub user: RwSignal<Option<User>>,
    pub token: RwSignal<Option<String>>,
    pub is_authenticated: Memo<bool>,
}

impl AuthState {
    pub fn new() -> Self {
        let token = create_rw_signal(
            LocalStorage::get::<String>("auth_token").ok()
        );
        let user = create_rw_signal(None);

        let is_authenticated = create_memo(move |_| {
            token.get().is_some() && user.get().is_some()
        });

        Self {
            user,
            token,
            is_authenticated,
        }
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<(), String> {
        // Call Ferriskey login endpoint
        let response = gloo_net::http::Request::post("http://localhost:8081/auth/login")
            .json(&serde_json::json!({
                "email": email,
                "password": password
            }))
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if response.ok() {
            let auth_token: AuthToken = response
                .json()
                .await
                .map_err(|e| e.to_string())?;

            // Store token
            LocalStorage::set("auth_token", &auth_token.access_token)
                .map_err(|e| e.to_string())?;

            self.token.set(Some(auth_token.access_token.clone()));

            // Fetch user info
            self.fetch_user_info().await?;

            Ok(())
        } else {
            Err("Invalid credentials".to_string())
        }
    }

    pub fn logout(&self) {
        LocalStorage::delete("auth_token");
        self.token.set(None);
        self.user.set(None);
    }

    async fn fetch_user_info(&self) -> Result<(), String> {
        // Fetch user info from backend using token
        // Implementation details...
        Ok(())
    }
}
```

---

### Task 11.4: Login & Registration Pages
**Estimation**: 3 hours
**Priority**: Critical
**Depends On**: Task 11.3

**Deliverables**:
- Create login page component
- Create registration page component
- Add form validation
- Integrate with Ferriskey authentication
- Add loading states and error handling
- Style authentication pages
- Add "Forgot Password" link (stub)

**File**: `frontend/src/pages/login.rs`

```rust
use leptos::*;
use crate::services::auth::AuthState;

#[component]
pub fn LoginPage() -> impl IntoView {
    let auth_state = use_context::<AuthState>()
        .expect("AuthState must be provided");

    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (error, set_error) = create_signal(None::<String>);
    let (loading, set_loading) = create_signal(false);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_loading.set(true);
        set_error.set(None);

        spawn_local(async move {
            match auth_state.login(&email.get(), &password.get()).await {
                Ok(_) => {
                    // Redirect to menu
                    leptos_router::use_navigate()("/menu", Default::default());
                }
                Err(e) => {
                    set_error.set(Some(e));
                    set_loading.set(false);
                }
            }
        });
    };

    view! {
        <div class="login-page">
            <h1>"Login to Royal Pizza"</h1>
            <form on:submit=on_submit>
                <div class="form-group">
                    <label for="email">"Email"</label>
                    <input
                        type="email"
                        id="email"
                        name="email"
                        required
                        on:input=move |ev| set_email.set(event_target_value(&ev))
                        prop:value=email
                    />
                </div>

                <div class="form-group">
                    <label for="password">"Password"</label>
                    <input
                        type="password"
                        id="password"
                        name="password"
                        required
                        on:input=move |ev| set_password.set(event_target_value(&ev))
                        prop:value=password
                    />
                </div>

                {move || error.get().map(|e| view! {
                    <div class="error">{e}</div>
                })}

                <button
                    type="submit"
                    disabled=move || loading.get()
                >
                    {move || if loading.get() { "Logging in..." } else { "Login" }}
                </button>
            </form>

            <p>
                "Don't have an account? "
                <a href="/register">"Register"</a>
            </p>
        </div>
    }
}
```

**File**: `frontend/src/pages/register.rs` (similar structure)

---

### Task 11.5: Protected Routes & Auth Guards
**Estimation**: 1 hour
**Priority**: High
**Depends On**: Task 11.4

**Deliverables**:
- Create route guard component
- Redirect unauthenticated users to login
- Add navigation bar with login/logout
- Show user info when authenticated
- Handle token expiration gracefully

**File**: `frontend/src/components/protected_route.rs`

```rust
use leptos::*;
use leptos_router::*;
use crate::services::auth::AuthState;

#[component]
pub fn ProtectedRoute(children: Children) -> impl IntoView {
    let auth_state = use_context::<AuthState>()
        .expect("AuthState must be provided");

    create_effect(move |_| {
        if !auth_state.is_authenticated.get() {
            let navigate = use_navigate();
            navigate("/login", Default::default());
        }
    });

    view! {
        <Show
            when=move || auth_state.is_authenticated.get()
            fallback=|| view! { <div>"Redirecting to login..."</div> }
        >
            {children()}
        </Show>
    }
}
```

**Update routing**:
```rust
<Route path="/order" view=move || view! {
    <ProtectedRoute>
        <OrderPage/>
    </ProtectedRoute>
}/>
```

---

## Phase 12: Authenticated Order Flow (8 hours)

### Task 12.1: Update Order Model with User Association
**Estimation**: 2 hours
**Priority**: Critical
**Depends On**: Phase 11

**Deliverables**:
- Add `user_id` field to Order model
- Update database schema to include user_id
- Add foreign key relationship (if supported)
- Update order creation to include authenticated user
- Remove customer name/phone from order form (use user profile)
- Migration script for existing orders

**File**: `shared/src/models/order.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub order_number: String,
    pub user_id: String,              // NEW: Associated user
    pub customer_name: String,         // From user profile
    pub customer_email: String,        // From user profile
    pub customer_phone: String,        // From user profile (or order form)
    pub items: Vec<OrderItem>,
    pub pickup_time: DateTime<Utc>,
    pub status: OrderStatus,
    pub total_amount: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

**Database Schema Update**:

**File**: `database/schema_v2.surql`

```sql
DEFINE TABLE order SCHEMAFULL;
DEFINE FIELD user_id ON TABLE order TYPE string ASSERT $value != NONE;
DEFINE FIELD order_number ON TABLE order TYPE string ASSERT $value != NONE;
DEFINE FIELD customer_name ON TABLE order TYPE string;
DEFINE FIELD customer_email ON TABLE order TYPE string;
DEFINE FIELD customer_phone ON TABLE order TYPE string;
DEFINE FIELD items ON TABLE order TYPE array;
DEFINE FIELD pickup_time ON TABLE order TYPE datetime;
DEFINE FIELD status ON TABLE order TYPE string;
DEFINE FIELD total_amount ON TABLE order TYPE number;
DEFINE FIELD created_at ON TABLE order TYPE datetime;
DEFINE FIELD updated_at ON TABLE order TYPE datetime;

DEFINE INDEX order_user_idx ON TABLE order COLUMNS user_id;
DEFINE INDEX order_number_idx ON TABLE order COLUMNS order_number UNIQUE;
```

---

### Task 12.2: Backend Order Endpoints with Authentication
**Estimation**: 2 hours
**Priority**: Critical
**Depends On**: Task 12.1

**Deliverables**:
- Update `create_order` handler to extract user from JWT
- Associate order with authenticated user automatically
- Fetch customer info from user profile or Ferriskey
- Update order validation logic
- Add authorization checks (users can only view their own orders)

**File**: `backend/src/handlers/order_handler.rs`

```rust
use axum::{Extension, Json};
use crate::middleware::auth::Claims;

pub async fn create_order(
    Extension(claims): Extension<Claims>,  // Injected by auth middleware
    State(state): State<AppState>,
    Json(request): Json<CreateOrderRequest>,
) -> Result<Json<CreateOrderResponse>, AppError> {
    // User is authenticated, extract user_id from claims
    let user_id = claims.sub;

    // Fetch user profile from Ferriskey or local cache
    let user_profile = fetch_user_profile(&user_id, &state).await?;

    // Create order with user info
    let order = Order {
        id: Uuid::new_v4().to_string(),
        order_number: generate_order_number(),
        user_id: user_id.clone(),
        customer_name: user_profile.name.clone(),
        customer_email: claims.email.clone(),
        customer_phone: request.phone.unwrap_or(user_profile.phone),
        items: request.items,
        pickup_time: request.pickup_time,
        status: OrderStatus::Pending,
        total_amount: calculate_total(&request.items),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Save to database
    let saved_order = state.order_repo.create_order(order).await?;

    Ok(Json(CreateOrderResponse {
        order_id: saved_order.id,
        order_number: saved_order.order_number,
        estimated_pickup: saved_order.pickup_time,
    }))
}

pub async fn get_order(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Path(order_id): Path<String>,
) -> Result<Json<Order>, AppError> {
    let order = state.order_repo.get_order_by_id(&order_id).await?;

    // Authorization: only allow user to view their own orders
    if order.user_id != claims.sub {
        return Err(AppError::Forbidden("You can only view your own orders".to_string()));
    }

    Ok(Json(order))
}
```

---

### Task 12.3: Frontend Order Flow Updates
**Estimation**: 2 hours
**Priority**: Critical
**Depends On**: Task 12.2

**Deliverables**:
- Update order page to not ask for name (use authenticated user)
- Simplify order form (only phone and pickup time)
- Pre-fill phone from user profile if available
- Add JWT token to all API requests
- Update API client to include Authorization header
- Handle 401 Unauthorized responses (redirect to login)

**File**: `frontend/src/api/client.rs`

```rust
use gloo_storage::{LocalStorage, Storage};

async fn get_auth_header() -> Option<String> {
    LocalStorage::get::<String>("auth_token")
        .ok()
        .map(|token| format!("Bearer {}", token))
}

pub async fn create_order(request: CreateOrderRequest) -> Result<CreateOrderResponse, String> {
    let auth_header = get_auth_header()
        .ok_or("Not authenticated")?;

    let response = gloo_net::http::Request::post("http://localhost:8080/api/orders")
        .header("Authorization", &auth_header)
        .json(&request)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status() == 401 {
        // Token expired, redirect to login
        return Err("Session expired. Please login again.".to_string());
    }

    if response.ok() {
        response.json().await.map_err(|e| e.to_string())
    } else {
        let error: ErrorResponse = response
            .json()
            .await
            .unwrap_or_else(|_| ErrorResponse {
                error: "Unknown error".to_string(),
            });
        Err(error.error)
    }
}
```

**Update Order Page**:

**File**: `frontend/src/pages/order.rs`

```rust
// Remove name input field
// Pre-fill phone from user profile
// Show user name from auth state

view! {
    <div class="order-page">
        <h1>"Complete Your Order"</h1>

        <div class="user-info">
            <p>"Ordering as: " {move || auth_state.user.get().map(|u| u.name)}</p>
        </div>

        <div class="cart-summary">
            // Cart items...
        </div>

        <form on:submit=on_submit>
            <div class="form-group">
                <label for="phone">"Phone Number"</label>
                <input
                    type="tel"
                    id="phone"
                    name="phone"
                    required
                    prop:value=phone  // Pre-filled from user profile
                    on:input=move |ev| set_phone.set(event_target_value(&ev))
                />
            </div>

            <div class="form-group">
                <label for="pickup-time">"Pickup Time"</label>
                // Date/time picker...
            </div>

            <button type="submit" disabled=move || loading.get()>
                "Place Order"
            </button>
        </form>
    </div>
}
```

---

### Task 12.4: User Profile Management
**Estimation**: 2 hours
**Priority**: Medium
**Depends On**: Task 12.3

**Deliverables**:
- Create user profile page
- Display user info (name, email, phone)
- Allow editing phone number
- Save profile updates to Ferriskey
- Add profile link to navigation

**File**: `frontend/src/pages/profile.rs`

```rust
#[component]
pub fn ProfilePage() -> impl IntoView {
    let auth_state = use_context::<AuthState>()
        .expect("AuthState must be provided");

    let (phone, set_phone) = create_signal(String::new());
    let (editing, set_editing) = create_signal(false);
    let (saving, set_saving) = create_signal(false);

    // Load user profile
    create_effect(move |_| {
        if let Some(user) = auth_state.user.get() {
            // Fetch full profile from backend
            // set_phone...
        }
    });

    let on_save = move |_| {
        set_saving.set(true);
        spawn_local(async move {
            // Update profile via API
            // ...
            set_saving.set(false);
            set_editing.set(false);
        });
    };

    view! {
        <div class="profile-page">
            <h1>"My Profile"</h1>

            <div class="profile-info">
                <div class="field">
                    <label>"Name"</label>
                    <span>{move || auth_state.user.get().and_then(|u| u.name)}</span>
                </div>

                <div class="field">
                    <label>"Email"</label>
                    <span>{move || auth_state.user.get().map(|u| u.email)}</span>
                </div>

                <div class="field">
                    <label>"Phone"</label>
                    <Show
                        when=move || editing.get()
                        fallback=move || view! {
                            <span>{phone}</span>
                            <button on:click=move |_| set_editing.set(true)>"Edit"</button>
                        }
                    >
                        <input
                            type="tel"
                            prop:value=phone
                            on:input=move |ev| set_phone.set(event_target_value(&ev))
                        />
                        <button on:click=on_save disabled=move || saving.get()>
                            "Save"
                        </button>
                        <button on:click=move |_| set_editing.set(false)>
                            "Cancel"
                        </button>
                    </Show>
                </div>
            </div>
        </div>
    }
}
```

---

## Phase 13: Order History & User Dashboard (10 hours)

### Task 13.1: Backend Order History Endpoint
**Estimation**: 2 hours
**Priority**: Critical
**Depends On**: Phase 12

**Deliverables**:
- Create `GET /api/orders/history` endpoint
- Return all orders for authenticated user
- Add filtering by status (optional query param)
- Add pagination support (page, limit)
- Sort by created_at descending (newest first)
- Include order items and details

**File**: `backend/src/handlers/order_handler.rs`

```rust
pub async fn get_user_orders(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Query(params): Query<OrderHistoryParams>,
) -> Result<Json<OrderHistoryResponse>, AppError> {
    let user_id = claims.sub;

    let orders = state
        .order_repo
        .get_orders_by_user(&user_id, params.status, params.page, params.limit)
        .await?;

    let total_count = state
        .order_repo
        .count_user_orders(&user_id, params.status)
        .await?;

    Ok(Json(OrderHistoryResponse {
        orders,
        total_count,
        page: params.page.unwrap_or(1),
        limit: params.limit.unwrap_or(10),
    }))
}

#[derive(Deserialize)]
pub struct OrderHistoryParams {
    pub status: Option<OrderStatus>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Serialize)]
pub struct OrderHistoryResponse {
    pub orders: Vec<Order>,
    pub total_count: u64,
    pub page: u32,
    pub limit: u32,
}
```

**Repository Implementation**:

**File**: `backend/src/repository/order_repo.rs`

```rust
pub async fn get_orders_by_user(
    &self,
    user_id: &str,
    status: Option<OrderStatus>,
    page: Option<u32>,
    limit: Option<u32>,
) -> Result<Vec<Order>, DbError> {
    let page = page.unwrap_or(1);
    let limit = limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let query = match status {
        Some(s) => format!(
            "SELECT * FROM order WHERE user_id = $user_id AND status = $status \
             ORDER BY created_at DESC LIMIT $limit START $offset"
        ),
        None => format!(
            "SELECT * FROM order WHERE user_id = $user_id \
             ORDER BY created_at DESC LIMIT $limit START $offset"
        ),
    };

    // Execute query and return results
    // ...
}
```

---

### Task 13.2: Frontend Order History Page
**Estimation**: 3 hours
**Priority**: Critical
**Depends On**: Task 13.1

**Deliverables**:
- Create order history page component
- Fetch and display user's orders
- Show order status with visual indicators
- Display order date, number, items, total
- Add filter by status dropdown
- Implement "View Details" for each order
- Add loading and empty states
- Style order cards

**File**: `frontend/src/pages/order_history.rs`

```rust
#[component]
pub fn OrderHistoryPage() -> impl IntoView {
    let (orders, set_orders) = create_signal(Vec::<Order>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (filter_status, set_filter_status) = create_signal(None::<OrderStatus>);

    // Fetch orders on mount and when filter changes
    create_effect(move |_| {
        let status = filter_status.get();
        set_loading.set(true);

        spawn_local(async move {
            match fetch_user_orders(status).await {
                Ok(response) => {
                    set_orders.set(response.orders);
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error.set(Some(e));
                    set_loading.set(false);
                }
            }
        });
    });

    view! {
        <div class="order-history-page">
            <h1>"My Orders"</h1>

            <div class="filters">
                <label>"Filter by status:"</label>
                <select on:change=move |ev| {
                    let value = event_target_value(&ev);
                    set_filter_status.set(match value.as_str() {
                        "all" => None,
                        "pending" => Some(OrderStatus::Pending),
                        "confirmed" => Some(OrderStatus::Confirmed),
                        "preparing" => Some(OrderStatus::Preparing),
                        "ready" => Some(OrderStatus::Ready),
                        "picked_up" => Some(OrderStatus::PickedUp),
                        "cancelled" => Some(OrderStatus::Cancelled),
                        _ => None,
                    });
                }>
                    <option value="all">"All Orders"</option>
                    <option value="pending">"Pending"</option>
                    <option value="confirmed">"Confirmed"</option>
                    <option value="preparing">"Preparing"</option>
                    <option value="ready">"Ready for Pickup"</option>
                    <option value="picked_up">"Picked Up"</option>
                    <option value="cancelled">"Cancelled"</option>
                </select>
            </div>

            <Show
                when=move || !loading.get()
                fallback=|| view! { <div class="loading">"Loading orders..."</div> }
            >
                <Show
                    when=move || !orders.get().is_empty()
                    fallback=|| view! { <div class="empty-state">"No orders yet"</div> }
                >
                    <div class="orders-list">
                        <For
                            each=move || orders.get()
                            key=|order| order.id.clone()
                            children=move |order| {
                                view! { <OrderCard order=order /> }
                            }
                        />
                    </div>
                </Show>
            </Show>
        </div>
    }
}

#[component]
fn OrderCard(order: Order) -> impl IntoView {
    view! {
        <div class="order-card">
            <div class="order-header">
                <h3>{order.order_number.clone()}</h3>
                <span class={format!("status status-{}", order.status.to_string().to_lowercase())}>
                    {order.status.to_string()}
                </span>
            </div>

            <div class="order-details">
                <p>"Date: " {order.created_at.format("%Y-%m-%d %H:%M").to_string()}</p>
                <p>"Pickup: " {order.pickup_time.format("%Y-%m-%d %H:%M").to_string()}</p>
                <p>"Items: " {order.items.len()}</p>
                <p class="total">"Total: $" {format!("{:.2}", order.total_amount)}</p>
            </div>

            <a href={format!("/order-details/{}", order.id)} class="view-details">
                "View Details"
            </a>
        </div>
    }
}
```

---

### Task 13.3: Order Details Page
**Estimation**: 2 hours
**Priority**: High
**Depends On**: Task 13.2

**Deliverables**:
- Create dedicated order details page
- Display full order information
- Show all ordered items with quantities and prices
- Display customer info and pickup time
- Show order status timeline/progress
- Add "Reorder" button (copy items to cart)
- Handle order not found scenario

**File**: `frontend/src/pages/order_details.rs`

```rust
#[component]
pub fn OrderDetailsPage() -> impl IntoView {
    let params = use_params_map();
    let order_id = move || params.get().get("id").cloned().unwrap_or_default();

    let (order, set_order) = create_signal(None::<Order>);
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);

    create_effect(move |_| {
        let id = order_id();
        set_loading.set(true);

        spawn_local(async move {
            match fetch_order_by_id(&id).await {
                Ok(o) => {
                    set_order.set(Some(o));
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error.set(Some(e));
                    set_loading.set(false);
                }
            }
        });
    });

    let on_reorder = move |_| {
        if let Some(order) = order.get() {
            // Add all items to cart
            let cart = use_context::<CartState>().expect("CartState must be provided");
            for item in order.items {
                cart.add_item(/* ... */);
            }

            // Navigate to menu
            leptos_router::use_navigate()("/menu", Default::default());
        }
    };

    view! {
        <div class="order-details-page">
            <Show
                when=move || !loading.get()
                fallback=|| view! { <div>"Loading..."</div> }
            >
                <Show
                    when=move || order.get().is_some()
                    fallback=move || view! {
                        <div class="error">
                            {error.get().unwrap_or("Order not found".to_string())}
                        </div>
                    }
                >
                    {move || order.get().map(|o| view! {
                        <div class="order-details">
                            <h1>"Order " {o.order_number.clone()}</h1>

                            <div class="status-timeline">
                                <OrderStatusTimeline status=o.status.clone() />
                            </div>

                            <div class="order-info">
                                <h2>"Order Information"</h2>
                                <p>"Order Date: " {o.created_at.format("%Y-%m-%d %H:%M").to_string()}</p>
                                <p>"Pickup Time: " {o.pickup_time.format("%Y-%m-%d %H:%M").to_string()}</p>
                                <p>"Status: " {o.status.to_string()}</p>
                            </div>

                            <div class="order-items">
                                <h2>"Order Items"</h2>
                                <For
                                    each=move || o.items.clone()
                                    key=|item| item.id.clone()
                                    children=move |item| {
                                        view! {
                                            <div class="item">
                                                <span>{item.description()}</span>
                                                <span>" x " {item.quantity}</span>
                                                <span>"$" {format!("{:.2}", item.subtotal)}</span>
                                            </div>
                                        }
                                    }
                                />
                                <div class="total">
                                    <strong>"Total: $" {format!("{:.2}", o.total_amount)}</strong>
                                </div>
                            </div>

                            <button on:click=on_reorder class="reorder-button">
                                "Reorder"
                            </button>
                        </div>
                    })}
                </Show>
            </Show>
        </div>
    }
}
```

---

### Task 13.4: User Dashboard
**Estimation**: 2 hours
**Priority**: Medium
**Depends On**: Task 13.3

**Deliverables**:
- Create user dashboard page (landing after login)
- Show recent orders (last 5)
- Display order statistics (total orders, favorite pizza)
- Quick actions: "Order Again", "View All Orders"
- Welcome message with user name
- Link to profile

**File**: `frontend/src/pages/dashboard.rs`

```rust
#[component]
pub fn DashboardPage() -> impl IntoView {
    let auth_state = use_context::<AuthState>()
        .expect("AuthState must be provided");

    let (recent_orders, set_recent_orders) = create_signal(Vec::<Order>::new());
    let (stats, set_stats) = create_signal(None::<OrderStats>);

    // Fetch recent orders and stats
    create_effect(move |_| {
        spawn_local(async move {
            // Fetch data...
        });
    });

    view! {
        <div class="dashboard">
            <h1>"Welcome, " {move || auth_state.user.get().and_then(|u| u.name)}</h1>

            <div class="quick-actions">
                <a href="/menu" class="action-card">
                    <h3>"Order Now"</h3>
                    <p>"Browse our menu"</p>
                </a>

                <a href="/order-history" class="action-card">
                    <h3>"Order History"</h3>
                    <p>"View past orders"</p>
                </a>

                <a href="/profile" class="action-card">
                    <h3>"Profile"</h3>
                    <p>"Manage your info"</p>
                </a>
            </div>

            <div class="recent-orders">
                <h2>"Recent Orders"</h2>
                // Display recent orders...
            </div>

            <div class="stats">
                <h2>"Your Stats"</h2>
                // Display statistics...
            </div>
        </div>
    }
}
```

---

### Task 13.5: Navigation & Layout Updates
**Estimation**: 1 hour
**Priority**: High
**Depends On**: Task 13.4

**Deliverables**:
- Update navigation bar with authenticated menu
- Show user name and avatar
- Add dropdown menu: Dashboard, Profile, Order History, Logout
- Update route structure to include new pages
- Add breadcrumbs for better navigation

**File**: `frontend/src/components/navbar.rs`

```rust
#[component]
pub fn NavBar() -> impl IntoView {
    let auth_state = use_context::<AuthState>()
        .expect("AuthState must be provided");

    let on_logout = move |_| {
        auth_state.logout();
        leptos_router::use_navigate()("/login", Default::default());
    };

    view! {
        <nav class="navbar">
            <div class="nav-brand">
                <a href="/">"Royal Pizza"</a>
            </div>

            <div class="nav-links">
                <a href="/menu">"Menu"</a>

                <Show
                    when=move || auth_state.is_authenticated.get()
                    fallback=|| view! {
                        <a href="/login">"Login"</a>
                        <a href="/register">"Register"</a>
                    }
                >
                    <div class="user-menu">
                        <button class="user-button">
                            {move || auth_state.user.get().and_then(|u| u.name).unwrap_or("User".to_string())}
                        </button>
                        <div class="dropdown">
                            <a href="/dashboard">"Dashboard"</a>
                            <a href="/order-history">"Order History"</a>
                            <a href="/profile">"Profile"</a>
                            <button on:click=on_logout>"Logout"</button>
                        </div>
                    </div>
                </Show>

                <div class="cart-icon">
                    <a href="/order">
                        "🛒 " {move || /* cart count */}
                    </a>
                </div>
            </div>
        </nav>
    }
}
```

---

## Phase 14: Documentation Updates (4 hours)

### Task 14.1: Update architecture.md
**Estimation**: 1.5 hours
**Priority**: High
**Depends On**: Phase 13

**Deliverables**:
- Document authentication flow with Ferriskey
- Update architecture diagrams
- Add JWT token flow explanation
- Document new endpoints (auth, order history)
- Update deployment architecture with Ferriskey
- Add security considerations section

**File**: `architecture.md`

**New Sections**:
```markdown
## Authentication Architecture

### Ferriskey Integration
Royal Pizza uses Ferriskey as its Identity and Access Management (IAM) system.

**Flow**:
1. User registers/logs in via Ferriskey
2. Ferriskey issues JWT access token
3. Frontend stores JWT in local storage
4. All API requests include JWT in Authorization header
5. Backend validates JWT on protected routes

### JWT Token Structure
```json
{
  "sub": "user-id-123",
  "email": "user@example.com",
  "exp": 1234567890,
  "iat": 1234567890
}
```

### Protected Endpoints
- `POST /api/orders` - Requires authentication
- `GET /api/orders/:id` - Requires authentication + ownership
- `GET /api/orders/history` - Requires authentication

### Security
- JWT secret stored as environment variable
- Tokens expire after 24 hours
- HTTPS required in production
- CORS configured for allowed origins only
```

---

### Task 14.2: Update business.md
**Estimation**: 1 hour
**Priority**: High
**Depends On**: Task 14.1

**Deliverables**:
- Document new user flows with authentication
- Update user stories
- Add order history feature description
- Document user profile management
- Update success metrics

**File**: `business.md`

**Updated User Stories**:
```markdown
## User Stories (Sprint 2)

### Authentication
- As a new user, I want to register an account so I can place orders
- As a returning user, I want to log in to access my order history
- As a logged-in user, I want my info saved so I don't re-enter it

### Order Management
- As an authenticated user, I want to view my past orders
- As a user, I want to filter my orders by status
- As a user, I want to reorder from my order history
- As a user, I want to see detailed info about each order

### Profile Management
- As a user, I want to view and edit my profile
- As a user, I want to update my phone number
- As a user, I want to logout securely
```

---

### Task 14.3: API Documentation
**Estimation**: 1 hour
**Priority**: Medium
**Depends On**: Task 14.2

**Deliverables**:
- Create comprehensive API documentation
- Document all endpoints with examples
- Add authentication requirements
- Document request/response formats
- Add error codes and messages
- Create OpenAPI/Swagger spec (optional)

**File**: `docs/API.md`

**Content**:
```markdown
# Royal Pizza API Documentation

## Authentication

All protected endpoints require a JWT token in the Authorization header:
```
Authorization: Bearer <token>
```

## Endpoints

### Authentication (via Ferriskey)
- `POST /auth/register` - Register new user
- `POST /auth/login` - Login and receive JWT
- `POST /auth/logout` - Logout (invalidate token)

### Pizzas (Public)
- `GET /api/pizzas` - List all available pizzas
- `GET /api/pizzas/:id` - Get pizza details

### Orders (Protected)
- `POST /api/orders` - Create new order (requires auth)
- `GET /api/orders/:id` - Get order details (requires auth + ownership)
- `GET /api/orders/history` - Get user's order history (requires auth)

### Health
- `GET /api/health` - Service health check

## Examples

### Create Order
**Request**:
```json
POST /api/orders
Authorization: Bearer eyJhbGc...

{
  "items": [
    {
      "pizza_id": "pizza-1",
      "size": "large",
      "quantity": 2
    }
  ],
  "phone": "+1234567890",
  "pickup_time": "2026-02-14T18:00:00Z"
}
```

**Response**:
```json
{
  "order_id": "order-123",
  "order_number": "RP-20260213-001",
  "estimated_pickup": "2026-02-14T18:00:00Z"
}
```
```

---

### Task 14.4: Deployment & Operations Guide
**Estimation**: 0.5 hours
**Priority**: Medium
**Depends On**: Task 14.3

**Deliverables**:
- Document production deployment steps
- Add environment variable setup guide
- Document Ferriskey configuration
- Add monitoring and logging guide
- Create troubleshooting section

**File**: `docs/DEPLOYMENT.md`

---

## Phase 15: Final Testing & Production Readiness (10 hours)

### Task 15.1: Security Audit
**Estimation**: 2 hours
**Priority**: Critical
**Depends On**: Phase 14

**Deliverables**:
- Review JWT implementation for vulnerabilities
- Check for SQL injection risks
- Verify input validation on all endpoints
- Test authorization checks (can't access other users' orders)
- Check for XSS vulnerabilities in frontend
- Verify CORS configuration
- Test rate limiting (if implemented)
- Scan dependencies for known vulnerabilities

**Tools**:
- `cargo audit` for Rust dependencies
- `npm audit` for JavaScript dependencies (Playwright)
- Manual security testing
- OWASP Top 10 checklist

**Security Checklist**:
- [ ] JWT secret is strong and environment-specific
- [ ] Passwords are hashed (handled by Ferriskey)
- [ ] SQL injection prevented (parameterized queries)
- [ ] Authorization checks on all protected routes
- [ ] Input validation on client and server
- [ ] HTTPS enforced in production
- [ ] CORS properly configured
- [ ] No sensitive data in logs
- [ ] Error messages don't leak information
- [ ] Dependencies up to date

---

### Task 15.2: Performance Testing
**Estimation**: 2 hours
**Priority**: High
**Depends On**: Task 15.1

**Deliverables**:
- Load test API endpoints
- Measure response times under load
- Test database performance with many orders
- Optimize slow queries
- Test frontend performance
- Measure WASM bundle size
- Add performance metrics

**Tools**:
- `wrk` or `artillery` for load testing
- Browser DevTools for frontend profiling
- `cargo bench` for Rust benchmarks

**Performance Targets**:
- API p95 < 100ms
- Menu load time < 1s
- Order creation < 200ms
- Support 100 concurrent users
- Frontend bundle < 2MB

---

### Task 15.3: E2E Test Suite Completion
**Estimation**: 3 hours
**Priority**: Critical
**Depends On**: Task 15.2

**Deliverables**:
- Run full Playwright test suite
- Fix any failing tests
- Add missing test coverage
- Test on all browsers (Chrome, Firefox, Safari)
- Test on mobile viewports
- Add CI/CD pipeline for automated tests
- Configure test reporting

**CI/CD Integration**:

**File**: `.github/workflows/test.yml`

```yaml
name: E2E Tests

on:
  push:
    branches: [master, develop]
  pull_request:
    branches: [master]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Set up Docker
        run: docker-compose up -d

      - name: Install Playwright
        working-directory: e2e
        run: npm ci && npx playwright install --with-deps

      - name: Run tests
        working-directory: e2e
        run: npx playwright test

      - name: Upload test results
        if: always()
        uses: actions/upload-artifact@v3
        with:
          name: playwright-report
          path: e2e/playwright-report/
```

---

### Task 15.4: Production Deployment
**Estimation**: 2 hours
**Priority**: Critical
**Depends On**: Task 15.3

**Deliverables**:
- Deploy to production environment
- Configure production environment variables
- Set up database backups
- Configure monitoring and alerts
- Test production deployment
- Smoke test all critical paths
- Set up logging aggregation

**Deployment Checklist**:
- [ ] Production environment variables configured
- [ ] JWT secret is unique and secure
- [ ] Database credentials are secure
- [ ] HTTPS certificates configured
- [ ] CORS origins set to production domain
- [ ] Database backups scheduled
- [ ] Monitoring and alerts configured
- [ ] Logs being collected
- [ ] Health checks passing
- [ ] All services starting correctly

**Monitoring Setup**:
- Health check endpoints monitored
- Error rate alerts
- Response time alerts
- Database connection alerts
- Disk space monitoring

---

### Task 15.5: User Acceptance Testing (UAT)
**Estimation**: 1 hour
**Priority**: High
**Depends On**: Task 15.4

**Deliverables**:
- Manual UAT testing of all features
- Test with real user scenarios
- Verify all user stories completed
- Check responsive design on devices
- Test error scenarios
- Gather feedback
- Document any issues for future sprints

**UAT Scenarios**:
1. New user registration and first order
2. Returning user login and reorder
3. Browse menu, add multiple items, checkout
4. View order history and filter
5. Update profile information
6. Mobile ordering experience
7. Error handling (network issues, etc.)

---

## Success Metrics (Sprint 2)

### Technical Metrics
- ✅ All Clippy warnings resolved
- ✅ E2E test coverage > 80%
- ✅ All security checks passing
- ✅ API response time p95 < 100ms
- ✅ Frontend load time < 1s
- ✅ Zero critical vulnerabilities

### Feature Metrics
- ✅ User authentication with Ferriskey working
- ✅ Orders associated with users
- ✅ Order history functional
- ✅ User profile management working
- ✅ E2E tests passing on all browsers

### Quality Metrics
- ✅ Code review completed
- ✅ Documentation updated
- ✅ Production deployment successful
- ✅ UAT completed with no blockers

---

## Execution Plan (Sprint 2)

### Week 1: Infrastructure & Authentication (Days 1-4)

**Day 1: Docker & Quality (7 hours)**
- Phase 8: Production Docker Configuration
- Phase 9: Code Quality & Refactoring

**Day 2-3: E2E Testing Setup (16 hours)**
- Phase 10: E2E Testing with Playwright
- Set up all test scenarios

**Day 4: IAM Setup (6 hours)**
- Phase 11 Tasks 11.1-11.2: Ferriskey integration

### Week 2: Features & Testing (Days 5-8)

**Day 5: Authentication UI (6 hours)**
- Phase 11 Tasks 11.3-11.5: Frontend auth

**Day 6: Authenticated Orders (8 hours)**
- Phase 12: Complete authenticated order flow

**Day 7-8: Order History & Polish (14 hours)**
- Phase 13: Order history feature
- Phase 14: Documentation updates

### Week 3: Final Testing & Deploy (Days 9-10)

**Day 9-10: Production Readiness (10 hours)**
- Phase 15: Security, performance, deployment

---

## Risk Mitigation (Sprint 2)

| Risk | Impact | Mitigation |
|------|--------|-----------|
| Ferriskey integration complexity | High | Start early, thorough testing |
| JWT security vulnerabilities | Critical | Security audit, follow best practices |
| E2E test flakiness | Medium | Proper waits, retry logic |
| Performance degradation | Medium | Load testing, optimization |
| Migration of existing orders | Low | Script to add user_id to existing orders |

---

## Post-Sprint 2 Enhancements

Future improvements:
- Real-time order status updates (WebSockets)
- Email notifications for order status
- Admin dashboard for order management
- Advanced analytics and reporting
- Loyalty program integration
- Social authentication (Google, Facebook)
- Order scheduling (future dates)
- Delivery option (in addition to pickup)
- Menu customization for dietary restrictions

---

## Notes

### Breaking Changes from Sprint 1
1. **Authentication Required**: Users must register/login to place orders
2. **Order Model**: Added `user_id` field (migration needed)
3. **API Changes**: All order endpoints now require authentication

### Migration Strategy
- Existing anonymous orders: assign to a placeholder "guest" user
- Or: mark old orders with `user_id: null` (handle in queries)

### Development Workflow
**Production**: `docker-compose up` (all services)
**Development**:
- `docker-compose -f docker-compose.dev.yml up database ferriskey`
- Terminal 1: `cd backend && cargo run`
- Terminal 2: `cd frontend && trunk serve`
- Terminal 3: E2E tests when needed

---

**Version**: 2.0
**Last Updated**: 2026-02-13
**Prepared By**: Claude Code
**Status**: Ready for Implementation (Sprint 2)