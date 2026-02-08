## Business View

The business starts small as an online Pizza Restaurant. Name "Royal Pizza"

---

## Version 1 - MVP Features

The first step is to deliver a service for a Web Page with the following features:

* Showing a card with 9 Pizzas and 1 option to create a custom Pizza
* Building an order with only a name and phone number without registration
* Pickup Date/Time for order

---

## POC Analysis & Reflections

### Core User Journey
1. **Browse Menu** → View 9 pre-defined pizzas + custom pizza option
2. **Select Pizza(s)** → Add to order with quantity
3. **Provide Details** → Name + Phone (unverified)
4. **Choose Pickup** → Date/Time selection
5. **Confirm Order** → Submit and receive confirmation with an order ID

### Critical Questions for POC

#### Menu & Products
- **Pizza Data Structure**
    - Name, description, price, image : currently fictive
    - Ingredients list for display : yes fictivr
    - Size options (S/M/L)

- **Custom Pizza**
    - Simple POC approach: Text field for special instructions

- Database
    - SurrealDB, it is part of the POC

#### Order Process
- **Shopping Cart**
    - Yes, maintain a simple cart with quantities

- **Order Validation** - What's required?
    - Phone format validation not needed
    - Pickup time constraints only business hours

- **Order Confirmation**
    - Display order number/confirmation ID
    - Show summary page

#### Pickup Scheduling
- **Time Slots**
    - Simple date/time picker with 30min minimum lead time

### Out of Scope for POC (Future Versions)
- ❌ Payment processing
- ❌ User accounts/registration
- ❌ Order tracking
- ❌ Admin panel for order management
- ❌ Email/SMS notifications
- ❌ Delivery option (pickup only)
- ❌ Complex custom pizza builder UI

### POC Success Criteria
✅ **Visual showcase**: Clean, responsive UI displaying 9 pizzas
✅ **Functional ordering**: End-to-end order placement without errors
✅ **Data persistence**: Orders saved to SurrealDB (or in-memory for initial POC)
✅ **Basic validation**: Name, phone, pickup time validation
✅ **Confirmation feedback**: User sees order was placed successfully

### Suggested POC Phases

#### Phase 1: Static Frontend (Days 1-2)
- Leptos frontend with hardcoded pizza data
- Display 9 pizza cards with mock data
- Basic order form (name, phone, pickup)
- No backend integration yet

#### Phase 2: Backend API (Days 3-4)
- Axum REST API endpoints:
    - `GET /api/pizzas` - List pizzas
    - `POST /api/orders` - Submit order
    - `GET /api/orders/:id` - Get order details
- In-memory storage first (Vec or HashMap)
- Input validation

#### Phase 3: Database Integration (Days 5-6)
- SurrealDB setup in Docker Compose
- Replace in-memory storage with DB persistence
- Schema design for pizzas and orders

#### Phase 4: Polish & Demo (Day 7)
- Error handling and user feedback
- Responsive design tweaks
- Docker Compose full stack deployment
- Demo preparation

### Technical Considerations
- **Leptos SSR vs CSR**: Client-side rendering sufficient for POC?
- **API Design**: RESTful principles, proper HTTP status codes
- **CORS**: Frontend-backend communication in Docker network
- **Data Validation**: Shared validation logic (Rust types across frontend/backend)
- **State Management**: Leptos signals for cart management

### Open Decisions Needed
1. Should we support multiple quantities per pizza in cart?
2. What's the minimum pickup lead time (30 mins? 1 hour?)?
3. Phone number format (US only? International?)?
4. Price display needed for POC? (Yes/No/Mock prices?)
5. Images for pizzas (URLs? Placeholder images? Icons?)

---

## Next Steps for POC
1. **Clarify open questions** above
2. **Set up workspace structure** (frontend, backend, database modules)
3. **Define data models** (Pizza, Order structs)
4. **Create mock pizza data** (9 pizzas with names, descriptions)
5. **Build frontend-first** (visible progress)
6. **Integrate backend** progressively
7. **Deploy** Docker Compose stack