# Royal Pizza - System Architecture

## Table of Contents
1. [Architecture Overview](#architecture-overview)
2. [System Composition](#system-composition)
3. [Technology Stack](#technology-stack)
4. [Data Models](#data-models)
5. [API Design](#api-design)
6. [Database Schema](#database-schema)
7. [Frontend Architecture](#frontend-architecture)
8. [Backend Architecture](#backend-architecture)
9. [Deployment Architecture](#deployment-architecture)
10. [Security & Performance](#security--performance)

---

## Architecture Overview

The application and all its components run in a Docker-Compose environment.

### System Diagram

```
┌─────────────────────────────────────────────────────┐
│                    User Browser                     │
│                 (Leptos WASM CSR)                   │
└──────────────────────┬──────────────────────────────┘
                       │ HTTP/REST
┌──────────────────────▼──────────────────────────────┐
│              Frontend Static Server                 │
│         (Axum serving static WASM/HTML)             │
└──────────────────────┬──────────────────────────────┘
                       │ API Calls
┌──────────────────────▼──────────────────────────────┐
│                Backend API Gateway                  │
│              (Axum RESTful Service)                 │
│  ┌──────────────┐  ┌──────────────┐                 │
│  │   Routes     │  │  Middleware  │                 │
│  │  Handlers    │  │  Validation  │                 │
│  └──────────────┘  └──────────────┘                 │
└──────────────────────┬──────────────────────────────┘
                       │ SurrealDB Protocol
┌──────────────────────▼──────────────────────────────┐
│              Data Persistence Layer                 │
│                   (SurrealDB)                       │
│  ┌──────────────┐  ┌──────────────┐                 │
│  │   Pizzas     │  │    Orders    │                 │
│  └──────────────┘  └──────────────┘                 │
└─────────────────────────────────────────────────────┘
```

---

## System Composition

* **Architecture Pattern**: Micro Service Architecture
* **Backend**: Axum RESTful Service
* **Frontend**: Leptos Client Application (CSR), running in an isolated Axum Server for static pages
* **Database**: SurrealDB
* **Container Orchestration**: Docker Compose

### Service Responsibilities

| Service | Port | Purpose |
|---------|------|---------|
| **Frontend Server** | 3000 | Serves static WASM/HTML/CSS |
| **Backend API** | 8080 | REST API endpoints |
| **SurrealDB** | 8000 | Data persistence |

---

## Rust Specifications

* All versions are stored only in the main `Cargo.toml` and provided to the modules as workspace dependencies. All cargo libraries are to take on the latest version existing on https://crates.io/
* Workspace structure enables shared code between frontend and backend. Shared code should be in a separate shared module.
* Single source of truth for dependency versions

### Workspace Configuration

```toml
# Cargo.toml (workspace root)
[workspace]
members = ["backend", "frontend", "shared"]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2024"
rust-version = "1.93"

[workspace.dependencies]
# Shared dependencies with versions defined once
# Core serialization
serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0.145"

# Date/time handling
chrono = { version = "0.4.43", features = ["serde"] }

# Async runtime
tokio = { version = "1.49.0", features = ["full"] }

# Backend framework
axum = "0.8.8"
tower = "0.5.3"
tower-http = { version = "0.6.8", features = ["cors", "fs", "trace"] }

# Frontend framework
leptos = "0.8.15"
leptos_router = "0.8.15"

# Database
surrealdb = "2.6.0"

# WASM dependencies
wasm-bindgen = "0.2.108"
web-sys = "0.3.85"
gloo-net = "0.6.0"

# Utilities
uuid = { version = "1.20.0", features = ["v4", "serde"] }
tracing = "0.1.44"
tracing-subscriber = { version = "0.3.20", features = ["env-filter"] }
```

---

## Technology Stack

### Frontend Stack
| Technology | Version | Purpose |
|------------|---------|---------|
| **Leptos** | 0.8.15 | Reactive UI framework (CSR mode) |
| **Leptos Router** | 0.8.15 | Client-side routing |
| **Trunk** | Latest | Build tool & WASM bundler |
| **wasm-bindgen** | 0.2.108 | JavaScript interop |
| **web-sys** | 0.3.85 | Web API bindings |
| **gloo-net** | 0.6.0 | HTTP client for WASM |

### Backend Stack
| Technology | Version | Purpose |
|------------|---------|---------|
| **Axum** | 0.8.8 | HTTP server & routing |
| **Tower** | 0.5.3 | Middleware & service composition |
| **Tower-HTTP** | 0.6.8 | CORS, logging middleware |
| **Tokio** | 1.49.0 | Async runtime |
| **Serde** | 1.0.228 | Serialization/deserialization |
| **Serde JSON** | 1.0.145 | JSON support |
| **UUID** | 1.20.0 | Unique ID generation |
| **Tracing** | 0.1.44 | Structured logging |
| **Tracing Subscriber** | 0.3.20 | Log collection & formatting |

### Database
| Technology | Version | Purpose |
|------------|---------|---------|
| **SurrealDB** | 2.6.0 | Multi-model database |

### DevOps
| Technology | Version | Purpose |
|------------|---------|---------|
| **Docker** | 24+ | Containerization |
| **Docker Compose** | 2.0+ | Multi-container orchestration |

---

## Data Models

### Core Domain Models

```rust
// shared/src/models/pizza.rs

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PizzaPrice {
    pub small: f64,
    pub medium: f64,
    pub large: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PizzaSize {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomPizza {
    pub instructions: String,
    pub size: PizzaSize,
}
```

```rust
// shared/src/models/order.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub order_number: String,  // e.g., "RP-20260208-001"
    pub customer: CustomerInfo,
    pub items: Vec<OrderItem>,
    pub pickup_time: DateTime<Utc>,
    pub status: OrderStatus,
    pub total_amount: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerInfo {
    pub name: String,
    pub phone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub id: String,
    pub item_type: OrderItemType,
    pub quantity: u32,
    pub unit_price: f64,
    pub subtotal: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderItemType {
    StandardPizza { pizza_id: String, size: PizzaSize },
    CustomPizza { custom: CustomPizza },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Preparing,
    Ready,
    PickedUp,
    Cancelled,
}
```

### Data Transfer Objects (DTOs)

```rust
// shared/src/dto/mod.rs

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub customer: CustomerInfo,
    pub items: Vec<OrderItemRequest>,
    pub pickup_time: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItemRequest {
    pub item_type: OrderItemType,
    pub quantity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderResponse {
    pub order_id: String,
    pub order_number: String,
    pub total_amount: f64,
    pub pickup_time: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub details: Option<Vec<String>>,
}
```

---

## API Design

### Base URL
```
Development: http://localhost:8080/api
Production: https://api.royalpizza.com/api
```

### Endpoints

#### 1. Pizza Endpoints

**GET /api/pizzas**
- **Purpose**: Retrieve all available pizzas
- **Response**: `200 OK`
```json
{
  "pizzas": [
    {
      "id": "pizza-1",
      "name": "Margherita",
      "description": "Classic tomato, mozzarella, and basil",
      "ingredients": ["Tomato Sauce", "Mozzarella", "Fresh Basil"],
      "price": {
        "small": 8.99,
        "medium": 12.99,
        "large": 15.99
      },
      "image_url": "/images/margherita.jpg",
      "is_available": true
    }
  ]
}
```

**GET /api/pizzas/:id**
- **Purpose**: Get specific pizza details
- **Response**: `200 OK` or `404 Not Found`

#### 2. Order Endpoints

**POST /api/orders**
- **Purpose**: Create a new order
- **Request Body**:
```json
{
  "customer": {
    "name": "John Doe",
    "phone": "555-0123"
  },
  "items": [
    {
      "item_type": {
        "StandardPizza": {
          "pizza_id": "pizza-1",
          "size": "Medium"
        }
      },
      "quantity": 2
    }
  ],
  "pickup_time": "2026-02-08T18:30:00Z"
}
```
- **Validation Rules**:
  - Name: 2-100 characters
  - Phone: Non-empty string
  - Pickup time: Minimum 30 minutes from now, during business hours
  - Items: At least 1 item
- **Response**: `201 Created`
```json
{
  "order_id": "order-uuid-123",
  "order_number": "RP-20260208-001",
  "total_amount": 25.98,
  "pickup_time": "2026-02-08T18:30:00Z"
}
```

**GET /api/orders/:id**
- **Purpose**: Retrieve order details for confirmation page
- **Response**: `200 OK` or `404 Not Found`

#### 3. Health Check

**GET /api/health**
- **Purpose**: Service health status
- **Response**: `200 OK`
```json
{
  "status": "healthy",
  "timestamp": "2026-02-08T10:00:00Z"
}
```

### Error Responses

All errors follow consistent format:
```json
{
  "error": "Validation failed",
  "details": [
    "Pickup time must be at least 30 minutes in the future",
    "Customer name is required"
  ]
}
```

**Status Codes**:
- `400 Bad Request`: Invalid input
- `404 Not Found`: Resource not found
- `422 Unprocessable Entity`: Validation errors
- `500 Internal Server Error`: Server errors

---

## Database Schema

### SurrealDB Schema Design

```sql
-- Database: royalpizza
-- Namespace: production

-- Pizza Table
DEFINE TABLE pizza SCHEMAFULL;

DEFINE FIELD id ON TABLE pizza TYPE string;
DEFINE FIELD name ON TABLE pizza TYPE string;
DEFINE FIELD description ON TABLE pizza TYPE string;
DEFINE FIELD ingredients ON TABLE pizza TYPE array;
DEFINE FIELD price ON TABLE pizza TYPE object;
DEFINE FIELD price.small ON TABLE pizza TYPE number;
DEFINE FIELD price.medium ON TABLE pizza TYPE number;
DEFINE FIELD price.large ON TABLE pizza TYPE number;
DEFINE FIELD image_url ON TABLE pizza TYPE option<string>;
DEFINE FIELD is_available ON TABLE pizza TYPE bool;
DEFINE FIELD created_at ON TABLE pizza TYPE datetime;
DEFINE FIELD updated_at ON TABLE pizza TYPE datetime;

DEFINE INDEX pizza_name_idx ON TABLE pizza COLUMNS name UNIQUE;

-- Order Table
DEFINE TABLE order SCHEMAFULL;

DEFINE FIELD id ON TABLE order TYPE string;
DEFINE FIELD order_number ON TABLE order TYPE string;
DEFINE FIELD customer ON TABLE order TYPE object;
DEFINE FIELD customer.name ON TABLE order TYPE string;
DEFINE FIELD customer.phone ON TABLE order TYPE string;
DEFINE FIELD items ON TABLE order TYPE array;
DEFINE FIELD pickup_time ON TABLE order TYPE datetime;
DEFINE FIELD status ON TABLE order TYPE string;
DEFINE FIELD total_amount ON TABLE order TYPE number;
DEFINE FIELD created_at ON TABLE order TYPE datetime;
DEFINE FIELD updated_at ON TABLE order TYPE datetime;

DEFINE INDEX order_number_idx ON TABLE order COLUMNS order_number UNIQUE;
DEFINE INDEX order_created_idx ON TABLE order COLUMNS created_at;
DEFINE INDEX order_pickup_idx ON TABLE order COLUMNS pickup_time;
```

### Initial Pizza Data (9 Standard Pizzas)

```rust
pub const INITIAL_PIZZAS: &[(&str, &str, &[&str], (f64, f64, f64))] = &[
    ("Margherita", "Classic tomato, mozzarella, and fresh basil",
     &["Tomato Sauce", "Mozzarella", "Fresh Basil"], (8.99, 12.99, 15.99)),

    ("Pepperoni", "Loaded with pepperoni and mozzarella",
     &["Tomato Sauce", "Mozzarella", "Pepperoni"], (9.99, 13.99, 16.99)),

    ("Hawaiian", "Ham and pineapple with mozzarella",
     &["Tomato Sauce", "Mozzarella", "Ham", "Pineapple"], (10.99, 14.99, 17.99)),

    ("Vegetarian", "Mixed vegetables and mozzarella",
     &["Tomato Sauce", "Mozzarella", "Bell Peppers", "Mushrooms", "Onions"], (10.99, 14.99, 17.99)),

    ("BBQ Chicken", "BBQ sauce with grilled chicken",
     &["BBQ Sauce", "Mozzarella", "Grilled Chicken", "Red Onions"], (11.99, 15.99, 18.99)),

    ("Meat Lovers", "Loaded with assorted meats",
     &["Tomato Sauce", "Mozzarella", "Pepperoni", "Sausage", "Ham", "Bacon"], (12.99, 16.99, 19.99)),

    ("Four Cheese", "Blend of four artisan cheeses",
     &["Mozzarella", "Parmesan", "Gorgonzola", "Ricotta"], (11.99, 15.99, 18.99)),

    ("Spicy Italian", "Italian sausage with hot peppers",
     &["Tomato Sauce", "Mozzarella", "Italian Sausage", "Jalapeños", "Red Pepper Flakes"], (11.99, 15.99, 18.99)),

    ("Mediterranean", "Feta, olives, and sun-dried tomatoes",
     &["Tomato Sauce", "Mozzarella", "Feta", "Kalamata Olives", "Sun-dried Tomatoes"], (11.99, 15.99, 18.99)),
];
```

---

## Frontend Architecture

### Component Structure

```
frontend/
├── src/
│   ├── main.rs                 # App entry point
│   ├── app.rs                  # Root App component
│   ├── routes.rs               # Route definitions
│   │
│   ├── pages/
│   │   ├── mod.rs
│   │   ├── menu.rs             # Menu/catalog page
│   │   ├── order.rs            # Order form page
│   │   └── confirmation.rs     # Success page
│   │
│   ├── components/
│   │   ├── mod.rs
│   │   ├── pizza_card.rs       # Pizza display card
│   │   ├── cart_item.rs        # Cart line item
│   │   ├── order_form.rs       # Customer details form
│   │   ├── pickup_scheduler.rs # Date/time picker
│   │   └── header.rs           # App header/nav
│   │
│   ├── state/
│   │   ├── mod.rs
│   │   ├── cart.rs             # Cart state management
│   │   └── order.rs            # Order state
│   │
│   ├── api/
│   │   ├── mod.rs
│   │   └── client.rs           # API client functions
│   │
│   └── utils/
│       ├── mod.rs
│       └── validation.rs       # Client-side validation
│
├── Cargo.toml
├── Trunk.toml
└── index.html
```

### State Management with Leptos Signals

```rust
// Using Leptos Signals for reactive state

#[derive(Clone, Debug)]
pub struct CartState {
    pub items: RwSignal<Vec<CartItem>>,
}

impl CartState {
    pub fn add_item(&self, item: CartItem) {
        self.items.update(|items| {
            if let Some(existing) = items.iter_mut()
                .find(|i| i.matches(&item)) {
                existing.quantity += item.quantity;
            } else {
                items.push(item);
            }
        });
    }

    pub fn total(&self) -> f64 {
        self.items.get().iter()
            .map(|item| item.subtotal())
            .sum()
    }
}
```

### Routing

```rust
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=MenuPage/>
                <Route path="/order" view=OrderPage/>
                <Route path="/confirmation/:id" view=ConfirmationPage/>
            </Routes>
        </Router>
    }
}
```

---

## Backend Architecture

### Server Structure

```
backend/
├── src/
│   ├── main.rs                 # Server entry point
│   ├── config.rs               # Configuration management
│   │
│   ├── routes/
│   │   ├── mod.rs
│   │   ├── pizzas.rs           # Pizza routes
│   │   └── orders.rs           # Order routes
│   │
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── pizza_handler.rs    # Pizza endpoint handlers
│   │   └── order_handler.rs    # Order endpoint handlers
│   │
│   ├── services/
│   │   ├── mod.rs
│   │   ├── pizza_service.rs    # Pizza business logic
│   │   └── order_service.rs    # Order business logic
│   │
│   ├── repository/
│   │   ├── mod.rs
│   │   ├── db.rs               # DB connection pool
│   │   ├── pizza_repo.rs       # Pizza data access
│   │   └── order_repo.rs       # Order data access
│   │
│   ├── middleware/
│   │   ├── mod.rs
│   │   ├── cors.rs             # CORS configuration
│   │   ├── logging.rs          # Request logging
│   │   └── error.rs            # Error handling
│   │
│   └── utils/
│       ├── mod.rs
│       └── id_generator.rs     # Order number generation
│
└── Cargo.toml
```

### Dependency Injection Pattern

```rust
use axum::{Extension, Router};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub pizza_service: Arc<PizzaService>,
    pub order_service: Arc<OrderService>,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .merge(pizza_routes())
        .merge(order_routes())
        .layer(Extension(state))
}
```

### Error Handling

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    ValidationError(Vec<String>),
    DatabaseError(String),
    InternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message, details) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg, None),
            AppError::ValidationError(errors) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "Validation failed".to_string(),
                Some(errors),
            ),
            AppError::DatabaseError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
                Some(vec![msg]),
            ),
            AppError::InternalError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal error".to_string(),
                Some(vec![msg]),
            ),
        };

        let body = Json(ErrorResponse {
            error: error_message,
            details,
        });

        (status, body).into_response()
    }
}
```

---

## Deployment Architecture

### Docker Compose Stack

```yaml
version: '3.9'

services:
  # SurrealDB Database
  database:
    image: surrealdb/surrealdb:latest
    ports:
      - "8000:8000"
    command: start --log trace --user root --pass root file:data/database.db
    volumes:
      - surrealdb_data:/data
    environment:
      - SURREAL_LOG=info
    networks:
      - royal_pizza_network
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8000/health"]
      interval: 10s
      timeout: 5s
      retries: 5

  # Backend API (Axum)
  backend:
    build:
      context: .
      dockerfile: backend/Dockerfile
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=ws://database:8000
      - DATABASE_NAMESPACE=royalpizza
      - DATABASE_NAME=production
      - RUST_LOG=info
      - PORT=8080
    depends_on:
      database:
        condition: service_healthy
    networks:
      - royal_pizza_network
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/api/health"]
      interval: 10s
      timeout: 5s
      retries: 5

  # Frontend (Leptos served via Axum)
  frontend:
    build:
      context: .
      dockerfile: frontend/Dockerfile
    ports:
      - "3000:3000"
    environment:
      - API_URL=http://backend:8080/api
    depends_on:
      - backend
    networks:
      - royal_pizza_network

volumes:
  surrealdb_data:

networks:
  royal_pizza_network:
    driver: bridge
```

### Container Build Files

**Backend Dockerfile**:
```dockerfile
FROM rust:1.93 as builder

WORKDIR /app
COPY . .

RUN cargo build --release --bin backend

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/backend /usr/local/bin/backend

EXPOSE 8080
CMD ["backend"]
```

**Frontend Dockerfile**:
```dockerfile
FROM rust:1.93 as builder

RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY . .

RUN cd frontend && trunk build --release

FROM rust:1.93
WORKDIR /app

COPY --from=builder /app/frontend/dist ./dist
COPY frontend/server.rs ./

RUN cargo init --name frontend_server
RUN echo 'axum = "0.7"' >> Cargo.toml
RUN echo 'tokio = { version = "1.35", features = ["full"] }' >> Cargo.toml
RUN echo 'tower = "0.4"' >> Cargo.toml
RUN echo 'tower-http = { version = "0.5", features = ["fs"] }' >> Cargo.toml

RUN cargo build --release

EXPOSE 3000
CMD ["cargo", "run", "--release"]
```

### Environment Configuration

```bash
# .env.development
DATABASE_URL=localhost:8000
DATABASE_NAMESPACE=royalpizza
DATABASE_NAME=development
RUST_LOG=debug
PORT=8080
CORS_ALLOW_ORIGIN=http://localhost:3000

# .env.production
DATABASE_URL=ws://database:8000
DATABASE_NAMESPACE=royalpizza
DATABASE_NAME=production
RUST_LOG=info
PORT=8080
CORS_ALLOW_ORIGIN=https://royalpizza.com
```

---

## Security & Performance

### Security Measures (POC)

1. **Input Validation**
   - Sanitize all user inputs (name, phone, custom pizza instructions)
   - Validate date/time constraints server-side
   - Limit string lengths to prevent abuse

2. **SQL Injection Prevention**
   - Use SurrealDB parameterized queries
   - Never concatenate user input into queries

3. **CORS Configuration**
   - Whitelist specific frontend origin
   - No wildcard CORS in production

4. **Future Considerations**
   - Rate limiting
   - HTTPS/TLS
   - Input sanitization for XSS prevention

### Performance Targets (POC)

| Metric | Target |
|--------|--------|
| Page Load Time | < 2 seconds |
| API Response Time | < 200ms (95th percentile) |
| Database Query Time | < 50ms average |
| Concurrent Users | 10-50 (POC scale) |

### Optimization Strategies

1. **Frontend**
   - WASM binary optimization via `wasm-opt`
   - Lazy loading for images
   - Minimize bundle size with `Trunk.toml` settings

2. **Backend**
   - Connection pooling for SurrealDB
   - Async/await for non-blocking I/O
   - Efficient serialization with Serde

3. **Database**
   - Indexed fields for common queries
   - File-based storage for POC (in-memory performance)

---

## Development Workflow

### Local Development Setup

```bash
# 1. Start SurrealDB
docker run --rm -p 8000:8000 surrealdb/surrealdb:latest \
  start --user root --pass root file:data/database.db

# 2. Run Backend
cd backend
cargo run

# 3. Run Frontend (separate terminal)
cd frontend
trunk serve --port 3000
```

### Workspace Structure

```
royal-pizza/
├── Cargo.toml              # Workspace manifest
├── docker-compose.yml
├── .env.development
├── .env.production
├── business.md
├── architecture.md
│
├── shared/                 # Shared types & utilities
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── models/
│       ├── dto/
│       └── validation/
│
├── backend/                # Axum API server
│   ├── Cargo.toml
│   ├── Dockerfile
│   └── src/
│       ├── main.rs
│       ├── routes/
│       ├── handlers/
│       ├── services/
│       ├── repository/
│       └── middleware/
│
└── frontend/               # Leptos web app
    ├── Cargo.toml
    ├── Dockerfile
    ├── Trunk.toml
    └── src/
        ├── main.rs
        ├── app.rs
        ├── pages/
        ├── components/
        ├── state/
        └── api/
```

---

## Next Steps

1. ✅ Define architecture and technical stack
2. 🔄 Initialize workspace structure
3. 🔄 Create shared data models
4. 🔄 Implement backend API with Axum
5. 🔄 Build frontend UI with Leptos
6. 🔄 Integrate SurrealDB
7. 🔄 Deploy via Docker Compose