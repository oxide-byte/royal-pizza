# Royal Pizza - Database Documentation

## Overview

This directory contains the SurrealDB schema and initialization scripts for the Royal Pizza ordering system. The database uses SurrealDB 2.6.0 with a SCHEMAFULL approach for data validation and integrity.

## Files

### schema.surql

Defines the database structure with:

- **Pizza Table**: Stores menu items with pricing, ingredients, and availability
  - Fields: id, name, description, ingredients, price (small/medium/large), image_url, is_available
  - Indexes: name (unique), availability

- **Order Table**: Stores customer orders with items and status
  - Fields: id, order_number, customer info, items, pickup_time, status, total_amount
  - Indexes: order_number (unique), created_at, pickup_time, status

### init.surql

Seeds the database with 9 standard pizzas:

1. **Margherita** - Classic Italian ($10.99/$14.99/$18.99)
2. **Pepperoni** - American Classic ($11.99/$15.99/$19.99)
3. **Hawaiian** - Sweet and Savory ($12.99/$16.99/$20.99)
4. **Vegetarian** - Garden Fresh ($11.99/$15.99/$19.99)
5. **BBQ Chicken** - Smoky and Sweet ($13.99/$17.99/$21.99)
6. **Meat Lovers** - Carnivore's Dream ($14.99/$18.99/$22.99)
7. **Four Cheese** - Cheese Lover's Paradise ($13.99/$17.99/$21.99)
8. **Spicy Italian** - Heat and Flavor ($12.99/$16.99/$20.99)
9. **Mediterranean** - Greek Inspired ($13.99/$17.99/$21.99)

## Database Seeding

The backend automatically seeds the database on startup if:

- `DATABASE_SEED=true` in environment variables (default)
- No pizzas exist in the database

### Manual Seeding

If you need to manually execute the scripts:

```bash
# Start SurrealDB
surreal start --user root --pass root file:///tmp/royalpizza.db

# In another terminal, run the scripts
surreal import --conn ws://localhost:8000 --user root --pass root --ns royalpizza --db development database/schema.surql
surreal import --conn ws://localhost:8000 --user root --pass root --ns royalpizza --db development database/init.surql
```

### Re-seeding

To force a re-seed (will delete existing data):

1. Stop the backend
2. Delete the SurrealDB data file or clear the tables
3. Restart the backend with `DATABASE_SEED=true`

The `init.surql` script includes a `DELETE` statement for the pizzas, making it safe to re-run.

## Database Configuration

Environment variables required:

```bash
DATABASE_URL=ws://localhost:8000
DATABASE_NAMESPACE=royalpizza
DATABASE_NAME=development
DATABASE_USERNAME=root
DATABASE_PASSWORD=root
DATABASE_SEED=true
```

## Schema Features

### Data Validation

- **Strong typing**: All fields have explicit types
- **Constraints**: Non-null checks, length limits, value ranges
- **Unique constraints**: Pizza names and order numbers must be unique
- **Array validation**: Items must have at least one element

### Indexes

Optimized queries for:
- Finding available pizzas
- Looking up orders by number
- Filtering orders by pickup time
- Tracking order status

### Timestamps

Automatic timestamp management:
- `created_at`: Set when record is created
- `updated_at`: Set when record is created (can be updated manually)

## Health Check

The backend provides a health check endpoint that verifies database connectivity:

```bash
curl http://localhost:8080/api/health
```

Response:
```json
{
  "status": "healthy",
  "service": "royal-pizza-backend",
  "database": "connected",
  "timestamp": "2026-02-11T21:00:00Z"
}
```

## Troubleshooting

### Database Connection Issues

If the backend fails to connect:

1. Verify SurrealDB is running: `curl http://localhost:8000/health`
2. Check DATABASE_URL environment variable
3. Verify credentials (username/password)
4. Check logs: `RUST_LOG=debug cargo run`

### Seeding Failures

If seeding fails:

1. Check if SurrealDB is accessible
2. Verify namespace and database name
3. Check logs for specific errors
4. Try manual seeding (see above)

### Data Consistency

The schema enforces:
- Pizza names must be unique
- Order numbers must be unique
- Prices must be non-negative
- Orders must have at least one item
- Customer names must be 2-100 characters

## Production Considerations

For production deployment:

1. **Change default password**: Update DATABASE_PASSWORD
2. **Persistent storage**: Use Docker volumes or file-based storage
3. **Backups**: Regular backups of SurrealDB data files
4. **Monitoring**: Use health check endpoint for monitoring
5. **Schema migrations**: Version control schema changes
6. **Disable auto-seed**: Set `DATABASE_SEED=false` after initial deployment

## SurrealDB Resources

- [SurrealDB Documentation](https://surrealdb.com/docs)
- [SurrealQL Guide](https://surrealdb.com/docs/surrealql)
- [Schema Definition](https://surrealdb.com/docs/surrealql/statements/define/table)

---

**Phase 5 Implementation**: Database Setup ✅
**Last Updated**: 2026-02-11
