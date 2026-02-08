# Royal Pizza - Development Storyboard

## Executive Summary

**Project**: Royal Pizza MVP - Online Pizza Ordering System
**Timeline**: 7 days (56 hours)
**Tech Stack**: Rust + Leptos + Axum + SurrealDB + Docker Compose
**Delivery**: Fully functional POC with ordering capability

---

## Project Phases Overview

| Phase | Duration | Description |
|-------|----------|-------------|
| **Phase 0: Setup** | 4 hours | Workspace initialization, dependencies, project structure |
| **Phase 1: Shared Models** | 3 hours | Data models, DTOs, validation logic |
| **Phase 2: Backend Foundation** | 8 hours | Axum server, API endpoints, database integration |
| **Phase 3: Frontend Core** | 12 hours | Leptos UI, routing, components, state management |
| **Phase 4: Integration** | 6 hours | API client, end-to-end flow, error handling |
| **Phase 5: Database** | 8 hours | SurrealDB schema, seed data, repository layer |
| **Phase 6: Docker & Deploy** | 6 hours | Dockerfiles, docker-compose, environment config |
| **Phase 7: Polish & Testing** | 9 hours | Error handling, validation, responsiveness, demo prep |

**Total Estimated Time**: 56 hours

---

## Phase 0: Workspace Setup (4 hours)

### Task 0.1: Initialize Workspace Structure
**Estimation**: 1.5 hours
**Priority**: Critical

**Deliverables**:
- Create workspace `Cargo.toml` with members: `backend`, `frontend`, `shared`
- Set up workspace.dependencies with exact versions from architecture.md
- Configure edition = "2024", rust-version = "1.93"
- Create module directories with basic `Cargo.toml` files

**Files Created**:
```
royal-pizza/
├── Cargo.toml (workspace)
├── shared/Cargo.toml
├── backend/Cargo.toml
└── frontend/Cargo.toml
```

**Success Criteria**:
- `cargo check` runs successfully
- All workspace members compile

---

### Task 0.2: Configure Backend Dependencies
**Estimation**: 1 hour
**Priority**: Critical

**Deliverables**:
- Configure `backend/Cargo.toml` with Axum, Tower, SurrealDB, Tokio, Serde
- Set up tracing and tracing-subscriber for logging
- Add UUID generation dependency
- Reference shared module as workspace dependency

**Key Dependencies**:
```toml
[dependencies]
axum = { workspace = true }
tower = { workspace = true }
tower-http = { workspace = true, features = ["cors", "trace"] }
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
surrealdb = { workspace = true }
uuid = { workspace = true, features = ["v4", "serde"] }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
shared = { path = "../shared" }
```

---

### Task 0.3: Configure Frontend Dependencies
**Estimation**: 1 hour
**Priority**: Critical

**Deliverables**:
- Configure `frontend/Cargo.toml` with Leptos, Leptos Router
- Add WASM dependencies: wasm-bindgen, web-sys, gloo-net
- Create `Trunk.toml` for build configuration
- Create `index.html` entry point
- Reference shared module

**Key Dependencies**:
```toml
[dependencies]
leptos = { workspace = true }
leptos_router = { workspace = true }
wasm-bindgen = { workspace = true }
web-sys = { workspace = true }
gloo-net = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
shared = { path = "../shared" }
```

---

### Task 0.4: Shared Module Setup
**Estimation**: 0.5 hours
**Priority**: Critical

**Deliverables**:
- Configure `shared/Cargo.toml` with Serde, Chrono, UUID
- Create `lib.rs` with module exports
- Set up folder structure for models, dto, validation

**Structure**:
```
shared/src/
├── lib.rs
├── models/
│   ├── mod.rs
│   ├── pizza.rs
│   └── order.rs
├── dto/
│   └── mod.rs
└── validation/
    └── mod.rs
```

---

## Phase 1: Shared Data Models (3 hours)

### Task 1.1: Pizza Domain Models
**Estimation**: 1 hour
**Priority**: Critical
**Depends On**: Task 0.4

**Deliverables**:
- Define `Pizza` struct with id, name, description, ingredients, price, image_url, is_available
- Define `PizzaPrice` struct with small, medium, large fields
- Define `PizzaSize` enum (Small, Medium, Large)
- Define `CustomPizza` struct with instructions and size
- Add Serde derive macros for serialization

**File**: `shared/src/models/pizza.rs`

**Code Structure**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pizza {
    pub id: String,
    pub name: String,
    pub description: String,
    pub ingredients: Vec<String>,
    pub price: PizzaPrice,
    pub image_url: Option<String>,
    pub is_available: bool,
}
```

---

### Task 1.2: Order Domain Models
**Estimation**: 1.5 hours
**Priority**: Critical
**Depends On**: Task 1.1

**Deliverables**:
- Define `Order` struct with id, order_number, customer, items, pickup_time, status, total_amount
- Define `CustomerInfo` struct with name and phone
- Define `OrderItem` struct with id, item_type, quantity, unit_price, subtotal
- Define `OrderItemType` enum (StandardPizza, CustomPizza)
- Define `OrderStatus` enum (Pending, Confirmed, Preparing, Ready, PickedUp, Cancelled)
- Add Chrono DateTime fields with proper timezone handling

**File**: `shared/src/models/order.rs`

---

### Task 1.3: Data Transfer Objects (DTOs)
**Estimation**: 0.5 hours
**Priority**: High
**Depends On**: Task 1.2

**Deliverables**:
- Define `CreateOrderRequest` DTO
- Define `OrderItemRequest` DTO
- Define `CreateOrderResponse` DTO
- Define `ErrorResponse` DTO
- Define `GetPizzasResponse` DTO

**File**: `shared/src/dto/mod.rs`

---

## Phase 2: Backend Foundation (8 hours)

### Task 2.1: Server Entry Point & Configuration
**Estimation**: 1 hour
**Priority**: Critical
**Depends On**: Task 0.2

**Deliverables**:
- Create `backend/src/main.rs` with Tokio runtime setup
- Create `backend/src/config.rs` with environment variable loading
- Configure tracing subscriber for structured logging
- Define `AppState` struct for dependency injection
- Add server startup with graceful shutdown

**Environment Variables**:
- `DATABASE_URL`: ws://localhost:8000
- `DATABASE_NAMESPACE`: royalpizza
- `DATABASE_NAME`: development
- `PORT`: 8080
- `CORS_ALLOW_ORIGIN`: http://localhost:3000

---

### Task 2.2: Database Connection & Repository
**Estimation**: 2 hours
**Priority**: Critical
**Depends On**: Task 2.1

**Deliverables**:
- Create `backend/src/repository/db.rs` with SurrealDB connection pool
- Create `backend/src/repository/pizza_repo.rs` with CRUD operations
- Create `backend/src/repository/order_repo.rs` with CRUD operations
- Implement async methods: `get_all_pizzas()`, `get_pizza_by_id()`, `create_order()`, `get_order_by_id()`
- Handle database connection errors gracefully

**Files**:
```
backend/src/repository/
├── mod.rs
├── db.rs
├── pizza_repo.rs
└── order_repo.rs
```

---

### Task 2.3: Error Handling Middleware
**Estimation**: 1 hour
**Priority**: High
**Depends On**: Task 2.1

**Deliverables**:
- Create `backend/src/middleware/error.rs` with `AppError` enum
- Implement `IntoResponse` for `AppError`
- Define error types: NotFound, ValidationError, DatabaseError, InternalError
- Create consistent JSON error response format
- Map errors to appropriate HTTP status codes

**File**: `backend/src/middleware/error.rs`

---

### Task 2.4: Pizza Service & Handlers
**Estimation**: 2 hours
**Priority**: Critical
**Depends On**: Task 2.2, Task 2.3

**Deliverables**:
- Create `backend/src/services/pizza_service.rs` with business logic
- Create `backend/src/handlers/pizza_handler.rs` with route handlers
- Implement `GET /api/pizzas` endpoint
- Implement `GET /api/pizzas/:id` endpoint
- Add proper error handling and response formatting

**Endpoints**:
- `GET /api/pizzas` → List all available pizzas
- `GET /api/pizzas/:id` → Get specific pizza by ID

**Files**:
```
backend/src/services/pizza_service.rs
backend/src/handlers/pizza_handler.rs
backend/src/routes/pizzas.rs
```

---

### Task 2.5: Order Service & Handlers
**Estimation**: 2 hours
**Priority**: Critical
**Depends On**: Task 2.4

**Deliverables**:
- Create `backend/src/services/order_service.rs` with order creation logic
- Create `backend/src/handlers/order_handler.rs` with route handlers
- Implement `POST /api/orders` endpoint with validation
- Implement `GET /api/orders/:id` endpoint
- Create order number generator (format: "RP-YYYYMMDD-NNN")
- Calculate order total amount
- Validate pickup time (minimum 30 minutes from now, business hours)
- Validate customer information

**Validation Rules**:
- Customer name: 2-100 characters
- Phone: non-empty string
- Pickup time: >= 30 minutes from now
- Items: at least 1 item

**Files**:
```
backend/src/services/order_service.rs
backend/src/handlers/order_handler.rs
backend/src/routes/orders.rs
backend/src/utils/id_generator.rs
```

---

## Phase 3: Frontend Core (12 hours)

### Task 3.1: Leptos App Setup & Routing
**Estimation**: 1.5 hours
**Priority**: Critical
**Depends On**: Task 0.3

**Deliverables**:
- Create `frontend/src/main.rs` with WASM entry point
- Create `frontend/src/app.rs` with root App component
- Create `frontend/src/routes.rs` with route definitions
- Configure Leptos Router with 3 routes:
  - `/` → MenuPage
  - `/order` → OrderPage
  - `/confirmation/:id` → ConfirmationPage
- Create basic page stubs

**Routes**:
```rust
<Router>
    <Routes>
        <Route path="/" view=MenuPage/>
        <Route path="/order" view=OrderPage/>
        <Route path="/confirmation/:id" view=ConfirmationPage/>
    </Routes>
</Router>
```

---

### Task 3.2: Cart State Management
**Estimation**: 2 hours
**Priority**: Critical
**Depends On**: Task 3.1

**Deliverables**:
- Create `frontend/src/state/cart.rs` with `CartState` struct
- Implement cart operations using Leptos RwSignal:
  - `add_item()`: Add pizza to cart or increment quantity
  - `remove_item()`: Remove item from cart
  - `update_quantity()`: Change item quantity
  - `clear()`: Empty cart
  - `total()`: Calculate total price
  - `item_count()`: Get total items in cart
- Define `CartItem` struct with pizza_id, size, quantity, price

**File**: `frontend/src/state/cart.rs`

---

### Task 3.3: API Client Module
**Estimation**: 1.5 hours
**Priority**: Critical
**Depends On**: Task 3.1

**Deliverables**:
- Create `frontend/src/api/client.rs` with HTTP client functions
- Implement API calls using gloo-net:
  - `fetch_pizzas() -> Result<Vec<Pizza>>`
  - `fetch_pizza_by_id(id) -> Result<Pizza>`
  - `create_order(request) -> Result<CreateOrderResponse>`
  - `fetch_order_by_id(id) -> Result<Order>`
- Handle JSON serialization/deserialization
- Handle HTTP errors and map to user-friendly messages

**API Base URL**: `http://localhost:8080/api` (configurable)

**File**: `frontend/src/api/client.rs`

---

### Task 3.4: Pizza Card Component
**Estimation**: 2 hours
**Priority**: Critical
**Depends On**: Task 3.2

**Deliverables**:
- Create `frontend/src/components/pizza_card.rs`
- Display pizza image (placeholder or URL)
- Display pizza name, description, ingredients
- Display prices for Small, Medium, Large
- Size selection buttons/dropdown
- "Add to Cart" button with quantity selector
- Handle "Add to Cart" click event
- Show visual feedback when item added

**Props**:
- `pizza: Pizza`
- `on_add_to_cart: Callback<(pizza_id, size, quantity)>`

**File**: `frontend/src/components/pizza_card.rs`

---

### Task 3.5: Custom Pizza Component
**Estimation**: 1.5 hours
**Priority**: High
**Depends On**: Task 3.4

**Deliverables**:
- Create custom pizza card component
- Text area for special instructions (max 500 characters)
- Size selection (Small, Medium, Large)
- Price display based on size (fixed pricing)
- "Add to Cart" button
- Validation for instructions (non-empty, max length)

**Pricing**:
- Small: $10.99
- Medium: $14.99
- Large: $17.99

**File**: `frontend/src/components/custom_pizza_card.rs`

---

### Task 3.6: Menu Page Implementation
**Estimation**: 2 hours
**Priority**: Critical
**Depends On**: Task 3.4, Task 3.5

**Deliverables**:
- Create `frontend/src/pages/menu.rs`
- Fetch pizzas from API on page load
- Display loading state while fetching
- Display 9 pizza cards in responsive grid (3 columns on desktop, 1 on mobile)
- Display custom pizza card
- Show cart summary in header/sidebar (item count, total)
- "Proceed to Order" button (navigates to /order)
- Handle API errors gracefully

**Layout**:
```
┌─────────────────────────────────────┐
│  Header: Royal Pizza | Cart: 3 items│
├─────────────────────────────────────┤
│  ┌──────┐  ┌──────┐  ┌──────┐      │
│  │Pizza1│  │Pizza2│  │Pizza3│      │
│  └──────┘  └──────┘  └──────┘      │
│  ┌──────┐  ┌──────┐  ┌──────┐      │
│  │Pizza4│  │Pizza5│  │Pizza6│      │
│  └──────┘  └──────┘  └──────┘      │
│  ┌──────┐  ┌──────┐  ┌──────┐      │
│  │Pizza7│  │Pizza8│  │Pizza9│      │
│  └──────┘  └──────┘  └──────┘      │
│  ┌─────────────────────────────┐   │
│  │  Custom Pizza (Your Way)    │   │
│  └─────────────────────────────┘   │
└─────────────────────────────────────┘
```

**File**: `frontend/src/pages/menu.rs`

---

### Task 3.7: Order Form Page
**Estimation**: 2.5 hours
**Priority**: Critical
**Depends On**: Task 3.6

**Deliverables**:
- Create `frontend/src/pages/order.rs`
- Display cart summary (items, quantities, prices, total)
- Customer information form:
  - Name input (required, 2-100 chars)
  - Phone input (required, non-empty)
- Pickup date/time picker:
  - Date selector (today or future dates)
  - Time selector (business hours, minimum 30 min from now)
- Form validation on client side
- "Place Order" button
- Submit order to API
- Handle loading state during submission
- Navigate to confirmation page on success
- Display errors on failure

**Validation**:
- Name: required, 2-100 characters
- Phone: required, non-empty
- Pickup time: must be at least 30 minutes from now
- Cart: must have at least 1 item

**File**: `frontend/src/pages/order.rs`

---

### Task 3.8: Confirmation Page
**Estimation**: 1 hour
**Priority**: High
**Depends On**: Task 3.7

**Deliverables**:
- Create `frontend/src/pages/confirmation.rs`
- Extract order ID from URL params
- Fetch order details from API
- Display success message
- Display order number (e.g., "RP-20260208-001")
- Display customer name and phone
- Display ordered items with quantities and prices
- Display pickup time
- Display total amount
- "Order Another" button (navigate back to menu, clear cart)

**File**: `frontend/src/pages/confirmation.rs`

---

## Phase 4: Integration & Error Handling (6 hours)

### Task 4.1: CORS Configuration
**Estimation**: 0.5 hours
**Priority**: Critical
**Depends On**: Task 2.1

**Deliverables**:
- Create `backend/src/middleware/cors.rs`
- Configure CORS with allowed origins from environment
- Allow methods: GET, POST, OPTIONS
- Allow headers: Content-Type, Authorization
- Enable credentials if needed

**File**: `backend/src/middleware/cors.rs`

---

### Task 4.2: Request Logging Middleware
**Estimation**: 0.5 hours
**Priority**: Medium
**Depends On**: Task 2.1

**Deliverables**:
- Create `backend/src/middleware/logging.rs`
- Log incoming requests (method, path, status, duration)
- Use tracing spans for request context
- Integrate with tower-http tracing layer

**File**: `backend/src/middleware/logging.rs`

---

### Task 4.3: Frontend Error Handling
**Estimation**: 2 hours
**Priority**: High
**Depends On**: Task 3.8

**Deliverables**:
- Create error boundary component
- Handle API errors in all pages
- Display user-friendly error messages
- Implement retry mechanism for transient failures
- Add error logging (console)
- Create `ErrorDisplay` component for consistent error UI

**Error Scenarios**:
- Network failure
- API returns 4xx/5xx
- Invalid response format
- Timeout

**File**: `frontend/src/components/error_display.rs`

---

### Task 4.4: Form Validation (Client & Server)
**Estimation**: 2 hours
**Priority**: High
**Depends On**: Task 2.5, Task 3.7

**Deliverables**:
- Create `shared/src/validation/mod.rs` with validation functions
- Validate customer name (length, characters)
- Validate phone number (non-empty)
- Validate pickup time (business hours, minimum lead time)
- Validate order items (at least 1 item, valid pizza IDs)
- Share validation logic between frontend and backend
- Return validation errors with field-specific messages

**Business Rules**:
- Name: 2-100 characters, alphanumeric + spaces
- Phone: non-empty string (no format validation for POC)
- Pickup time: >= 30 minutes from now, within business hours (9 AM - 9 PM)

**File**: `shared/src/validation/mod.rs`

---

### Task 4.5: End-to-End Flow Testing
**Estimation**: 1 hour
**Priority**: High
**Depends On**: Task 4.4

**Deliverables**:
- Manual testing of complete user journey
- Test: Browse menu → Add items → Fill form → Submit → View confirmation
- Test error scenarios: invalid input, API failures, empty cart
- Verify data persistence in database
- Test with different pizza sizes and quantities
- Test custom pizza ordering

**Test Cases**:
1. Happy path: Order 2 pizzas, submit, verify confirmation
2. Validation errors: Submit with empty name, invalid pickup time
3. API failure: Stop backend, verify error handling
4. Empty cart: Try to order without items
5. Custom pizza: Order custom pizza with instructions

---

## Phase 5: Database Setup (8 hours)

### Task 5.1: SurrealDB Schema Definition
**Estimation**: 2 hours
**Priority**: Critical
**Depends On**: Task 1.2

**Deliverables**:
- Create `database/schema.surql` with table definitions
- Define `pizza` table schema (SCHEMAFULL)
- Define `order` table schema (SCHEMAFULL)
- Define field types matching Rust models
- Create indexes: pizza_name_idx, order_number_idx, order_created_idx, order_pickup_idx
- Document schema in comments

**Schema Features**:
- Strong typing with SurrealDB type system
- Unique constraints on pizza names and order numbers
- DateTime fields for created_at, updated_at, pickup_time
- Nested objects for price and customer info

**File**: `database/schema.surql`

---

### Task 5.2: Database Initialization Script
**Estimation**: 1 hour
**Priority**: High
**Depends On**: Task 5.1

**Deliverables**:
- Create `database/init.surql` with initial data
- Insert 9 standard pizzas with realistic data
- Set all pizzas to available
- Add timestamps
- Ensure script is idempotent (can be run multiple times)

**9 Standard Pizzas**:
1. Margherita - Classic tomato, mozzarella, and fresh basil
2. Pepperoni - Loaded with pepperoni and mozzarella
3. Hawaiian - Ham and pineapple with mozzarella
4. Vegetarian - Mixed vegetables and mozzarella
5. BBQ Chicken - BBQ sauce with grilled chicken
6. Meat Lovers - Loaded with assorted meats
7. Four Cheese - Blend of four artisan cheeses
8. Spicy Italian - Italian sausage with hot peppers
9. Mediterranean - Feta, olives, and sun-dried tomatoes

**File**: `database/init.surql`

---

### Task 5.3: Repository Implementation with Real DB
**Estimation**: 3 hours
**Priority**: Critical
**Depends On**: Task 5.2

**Deliverables**:
- Implement `PizzaRepository` methods with SurrealDB queries
- Implement `OrderRepository` methods with SurrealDB queries
- Use parameterized queries to prevent SQL injection
- Handle database connection errors
- Implement transaction support for order creation
- Add query logging for debugging

**Methods**:
- `get_all_pizzas()` → SELECT * FROM pizza WHERE is_available = true
- `get_pizza_by_id(id)` → SELECT * FROM pizza WHERE id = $id
- `create_order(order)` → INSERT INTO order {...}
- `get_order_by_id(id)` → SELECT * FROM order WHERE id = $id

**Files**: Update `backend/src/repository/pizza_repo.rs`, `backend/src/repository/order_repo.rs`

---

### Task 5.4: Database Seeding & Migration
**Estimation**: 1 hour
**Priority**: High
**Depends On**: Task 5.3

**Deliverables**:
- Create `backend/src/repository/seed.rs` with seeding function
- Implement one-time database initialization
- Check if data already exists before seeding
- Run schema.surql and init.surql on startup (development only)
- Add CLI flag or environment variable to control seeding

**File**: `backend/src/repository/seed.rs`

---

### Task 5.5: Database Connection Health Check
**Estimation**: 1 hour
**Priority**: Medium
**Depends On**: Task 5.3

**Deliverables**:
- Implement `GET /api/health` endpoint
- Check database connectivity
- Return JSON with status and timestamp
- Use in Docker health checks
- Log health check results

**Response Format**:
```json
{
  "status": "healthy",
  "database": "connected",
  "timestamp": "2026-02-08T10:00:00Z"
}
```

**File**: `backend/src/handlers/health.rs`

---

## Phase 6: Docker & Deployment (6 hours)

### Task 6.1: Backend Dockerfile
**Estimation**: 1.5 hours
**Priority**: Critical
**Depends On**: Task 2.5

**Deliverables**:
- Create `backend/Dockerfile` with multi-stage build
- Stage 1: Build backend with cargo build --release
- Stage 2: Minimal runtime image with Debian slim
- Install CA certificates and OpenSSL
- Copy binary from builder
- Expose port 8080
- Set CMD to run backend binary

**Optimizations**:
- Use Rust 1.93 as builder
- Strip debug symbols for smaller binary
- Minimal runtime dependencies

**File**: `backend/Dockerfile`

---

### Task 6.2: Frontend Dockerfile
**Estimation**: 2 hours
**Priority**: Critical
**Depends On**: Task 3.8

**Deliverables**:
- Create `frontend/Dockerfile` with multi-stage build
- Stage 1: Install Trunk, build WASM with trunk build --release
- Stage 2: Create simple Axum static file server
- Copy dist/ folder from builder
- Serve static files on port 3000
- Create `frontend/server.rs` for static serving

**Server Features**:
- Serve index.html, .wasm, .js, .css files
- Enable SPA routing (fallback to index.html)
- CORS headers if needed

**Files**:
- `frontend/Dockerfile`
- `frontend/server.rs`

---

### Task 6.3: Docker Compose Configuration
**Estimation**: 1.5 hours
**Priority**: Critical
**Depends On**: Task 6.1, Task 6.2

**Deliverables**:
- Create `docker-compose.yml` with 3 services:
  1. `database`: SurrealDB with file-based storage
  2. `backend`: Axum API server
  3. `frontend`: Leptos web app
- Configure service dependencies and health checks
- Set up named volumes for database persistence
- Configure network for service communication
- Map ports: 8000 (DB), 8080 (API), 3000 (Web)
- Set environment variables from .env files

**Health Checks**:
- Database: curl http://localhost:8000/health
- Backend: curl http://localhost:8080/api/health

**File**: `docker-compose.yml`

---

### Task 6.4: Environment Configuration
**Estimation**: 1 hour
**Priority**: High
**Depends On**: Task 6.3

**Deliverables**:
- Create `.env.development` for local development
- Create `.env.production` for Docker deployment
- Document environment variables in README
- Add `.env.example` template
- Configure database URLs, ports, CORS origins

**Variables**:
- `DATABASE_URL`: ws://database:8000
- `DATABASE_NAMESPACE`: royalpizza
- `DATABASE_NAME`: production
- `RUST_LOG`: info
- `PORT`: 8080
- `CORS_ALLOW_ORIGIN`: http://localhost:3000

**Files**:
- `.env.development`
- `.env.production`
- `.env.example`

---

## Phase 7: Polish & Testing (9 hours)

### Task 7.1: Responsive Design Implementation
**Estimation**: 3 hours
**Priority**: High
**Depends On**: Task 3.8

**Deliverables**:
- Add CSS/styling for mobile, tablet, desktop breakpoints
- Responsive grid for pizza cards (1-2-3 columns)
- Mobile-friendly forms with proper input types
- Touch-friendly buttons and controls
- Test on different screen sizes
- Add basic loading spinners and transitions

**Breakpoints**:
- Mobile: < 768px (1 column)
- Tablet: 768px - 1024px (2 columns)
- Desktop: > 1024px (3 columns)

**File**: `frontend/index.html` or `frontend/src/styles.css`

---

### Task 7.2: Loading States & User Feedback
**Estimation**: 2 hours
**Priority**: High
**Depends On**: Task 7.1

**Deliverables**:
- Add loading spinners for API calls
- Disable buttons during async operations
- Show success messages (toast/notification)
- Add animation for "Add to Cart" feedback
- Implement optimistic UI updates where appropriate
- Add skeleton screens for loading states

**Components**:
- `LoadingSpinner` component
- `Toast` notification component
- Skeleton cards for menu loading

**Files**:
- `frontend/src/components/loading.rs`
- `frontend/src/components/toast.rs`

---

### Task 7.3: Error Scenarios & Edge Cases
**Estimation**: 2 hours
**Priority**: High
**Depends On**: Task 7.2

**Deliverables**:
- Test and handle network failures
- Test with backend down
- Test with database down
- Handle invalid order IDs in confirmation page
- Handle stale data (pizza no longer available)
- Test concurrent orders
- Test with invalid pickup times

**Test Matrix**:
| Scenario | Expected Behavior |
|----------|-------------------|
| Network error | Show error message, allow retry |
| Backend down | Show maintenance message |
| Invalid order ID | Show "Order not found" page |
| Past pickup time | Validation error |
| Empty cart | Disable "Proceed to Order" |
| API timeout | Show timeout message, retry option |

---

### Task 7.4: Final Integration Testing
**Estimation**: 1.5 hours
**Priority**: Critical
**Depends On**: Task 7.3

**Deliverables**:
- Run full stack in Docker Compose
- Test complete user journey end-to-end
- Verify data persistence across container restarts
- Test with multiple concurrent users (simulate)
- Verify all API endpoints return correct data
- Check logs for errors or warnings
- Performance test: page load times, API response times

**Success Criteria**:
- All pages load in < 2 seconds
- API responses in < 200ms
- No console errors
- Database persists data correctly

---

### Task 7.5: Documentation & Demo Preparation
**Estimation**: 0.5 hours
**Priority**: Medium
**Depends On**: Task 7.4

**Deliverables**:
- Update README.md with:
  - Project overview
  - Architecture diagram
  - Setup instructions
  - Running with Docker Compose
  - Development workflow
  - API documentation link
- Create demo script with happy path
- Prepare sample orders for demo
- Screenshot key pages for documentation

**File**: `README.md`

---

## Execution Plan for Claude.ai

### Pre-Development Checklist
- [ ] Confirm Rust 1.93+ installed
- [ ] Confirm Docker and Docker Compose installed
- [ ] Confirm Trunk installed (`cargo install trunk`)
- [ ] Confirm wasm32-unknown-unknown target added (`rustup target add wasm32-unknown-unknown`)
- [ ] Review architecture.md and business.md

---

### Development Order (Optimized for Claude.ai)

**Week 1: Foundation (Days 1-3)**

**Day 1: Workspace & Models (7 hours)**
- Execute Phase 0 tasks (0.1 → 0.4) in sequence
- Execute Phase 1 tasks (1.1 → 1.3) in sequence
- Verify: `cargo check` passes for all workspace members
- Commit: "chore: initialize workspace and data models"

**Day 2: Backend API (8 hours)**
- Execute Phase 2 tasks (2.1 → 2.5) in sequence
- Test backend with curl or Postman
- Verify: All endpoints return expected responses
- Commit: "feat: implement backend API with Axum"

**Day 3: Database Integration (8 hours)**
- Execute Phase 5 tasks (5.1 → 5.5) in sequence
- Start SurrealDB locally
- Run schema and seed scripts
- Test backend with real database
- Verify: Data persists and queries work
- Commit: "feat: integrate SurrealDB with schema and seed data"

---

**Week 2: Frontend & Polish (Days 4-7)**

**Day 4: Frontend Foundation (8 hours)**
- Execute Phase 3 tasks (3.1 → 3.3) in sequence
- Execute Task 3.4 (Pizza Card Component)
- Verify: Pages render, routing works, API client functional
- Commit: "feat: implement Leptos frontend foundation and routing"

**Day 5: Frontend UI (8 hours)**
- Execute Phase 3 tasks (3.5 → 3.8) in sequence
- Test UI interactions locally
- Verify: Complete user flow from menu to confirmation
- Commit: "feat: complete frontend UI with all pages and components"

**Day 6: Integration & Docker (8 hours)**
- Execute Phase 4 tasks (4.1 → 4.5) in sequence
- Execute Phase 6 tasks (6.1 → 6.4) in sequence
- Test full stack in Docker Compose
- Verify: All services start and communicate
- Commit: "feat: add CORS, error handling, and Docker deployment"

**Day 7: Polish & Demo (9 hours)**
- Execute Phase 7 tasks (7.1 → 7.5) in sequence
- Run comprehensive tests
- Fix any bugs found
- Prepare demo
- Verify: All acceptance criteria met
- Commit: "chore: polish UI, add responsive design, and finalize documentation"

---

### Parallel Execution Opportunities

**Can be done in parallel (same developer):**
- Phase 1 (Models) ← No dependencies
- Phase 5.1 (Schema) ← Can start while working on backend
- Phase 6.4 (Environment) ← Can prepare anytime

**Cannot be parallelized:**
- Backend must be complete before frontend integration
- Database must be ready before repository implementation
- Dockerfiles depend on working code

---

### Critical Path

The critical path for MVP delivery:

```
Phase 0 → Phase 1 → Phase 2 → Phase 5 → Phase 3 → Phase 4 → Phase 6 → Phase 7
  (4h)     (3h)      (8h)      (8h)      (12h)     (6h)      (6h)      (9h)
```

**Total Critical Path Time**: 56 hours

---

### Risk Mitigation

| Risk | Impact | Mitigation Strategy |
|------|--------|---------------------|
| SurrealDB version compatibility | High | Use exact version 2.6.0, test early |
| WASM build issues | Medium | Test Trunk build in Phase 0.3 |
| Docker networking | Medium | Use docker-compose networks, test early |
| CORS errors | Low | Configure properly in Phase 4.1 |
| Leptos CSR limitations | Medium | Test API calls early in Phase 3.3 |

---

### Success Metrics

**Technical Metrics**:
- ✅ All Cargo.toml files compile without errors
- ✅ Backend API endpoints return 2xx responses
- ✅ Frontend builds WASM successfully with Trunk
- ✅ Docker Compose starts all 3 services
- ✅ Database persists data across restarts
- ✅ End-to-end order flow completes successfully

**Business Metrics**:
- ✅ User can browse 9 pizzas + custom option
- ✅ User can add items to cart with quantities
- ✅ User can submit order with name, phone, pickup time
- ✅ User receives order confirmation with order number
- ✅ Orders are saved to database

**Quality Metrics**:
- ✅ Page load time < 2 seconds
- ✅ API response time < 200ms (p95)
- ✅ No console errors in browser
- ✅ Responsive design works on mobile and desktop
- ✅ Error messages are user-friendly

---

## Post-MVP Enhancements (Out of Scope)

For future versions:
- User authentication and order history
- Payment integration (Stripe, PayPal)
- Real-time order status tracking
- Admin panel for order management
- Email/SMS notifications
- Delivery option (not just pickup)
- Advanced custom pizza builder (drag-and-drop ingredients)
- Reviews and ratings
- Promotional codes and discounts
- Analytics dashboard

---

## Notes for Claude.ai Implementation

### Best Practices
1. **Commit frequently**: After each task completion
2. **Test incrementally**: Verify each component before moving to next
3. **Use workspace dependencies**: Single source of truth for versions
4. **Shared code first**: Define models in `shared/` before using in `backend/` or `frontend/`
5. **Error handling**: Always return Result types, never panic in production code
6. **Logging**: Use tracing macros (info!, debug!, error!) throughout backend
7. **Type safety**: Leverage Rust's type system for compile-time guarantees

### Common Pitfalls to Avoid
- ❌ Don't hardcode API URLs in frontend (use environment variables)
- ❌ Don't use `unwrap()` or `expect()` in backend handlers
- ❌ Don't forget CORS configuration (will cause frontend API calls to fail)
- ❌ Don't forget to add `#[derive(Serialize, Deserialize)]` to all DTOs
- ❌ Don't use blocking I/O in async functions
- ❌ Don't forget to validate input on both client and server side

### Debugging Tips
- Use `RUST_LOG=debug` to see detailed logs
- Use browser DevTools Network tab to inspect API calls
- Use `docker-compose logs <service>` to view container logs
- Use `surreal sql` CLI to query database directly
- Use `trunk serve --open` for live reload during frontend development

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-02-08 | Initial storyboard based on architecture.md and business.md |

---

**Last Updated**: 2026-02-08
**Prepared By**: Claude Code
**Status**: Ready for Implementation