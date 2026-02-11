-- Royal Pizza Database Schema
-- SurrealDB Schema Definition for Pizza Ordering System
-- Version: 1.0
-- Date: 2026-02-11

-- ============================================================================
-- PIZZA TABLE
-- ============================================================================
-- Stores available pizza menu items with pricing and details

DEFINE TABLE pizza SCHEMAFULL;

-- Pizza fields
DEFINE FIELD id ON TABLE pizza TYPE string;
DEFINE FIELD name ON TABLE pizza TYPE string
    ASSERT $value != NONE AND string::len($value) > 0;
DEFINE FIELD description ON TABLE pizza TYPE string
    ASSERT $value != NONE;
DEFINE FIELD ingredients ON TABLE pizza TYPE array<string>
    ASSERT $value != NONE;
DEFINE FIELD price ON TABLE pizza TYPE object;
DEFINE FIELD price.small ON TABLE pizza TYPE number
    ASSERT $value >= 0;
DEFINE FIELD price.medium ON TABLE pizza TYPE number
    ASSERT $value >= 0;
DEFINE FIELD price.large ON TABLE pizza TYPE number
    ASSERT $value >= 0;
DEFINE FIELD image_url ON TABLE pizza TYPE option<string>;
DEFINE FIELD is_available ON TABLE pizza TYPE bool
    DEFAULT true;
DEFINE FIELD created_at ON TABLE pizza TYPE datetime
    DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE pizza TYPE datetime
    DEFAULT time::now();

-- Indexes for pizza table
DEFINE INDEX pizza_name_idx ON TABLE pizza COLUMNS name UNIQUE;
DEFINE INDEX pizza_available_idx ON TABLE pizza COLUMNS is_available;

-- ============================================================================
-- ORDER TABLE
-- ============================================================================
-- Stores customer orders with items, pickup times, and status tracking

DEFINE TABLE order SCHEMAFULL;

-- Order fields
DEFINE FIELD id ON TABLE order TYPE string;
DEFINE FIELD order_number ON TABLE order TYPE string
    ASSERT $value != NONE AND string::len($value) > 0;
DEFINE FIELD customer ON TABLE order TYPE object;
DEFINE FIELD customer.name ON TABLE order TYPE string
    ASSERT $value != NONE AND string::len($value) >= 2 AND string::len($value) <= 100;
DEFINE FIELD customer.phone ON TABLE order TYPE string
    ASSERT $value != NONE AND string::len($value) > 0;
DEFINE FIELD items ON TABLE order TYPE array<object>
    ASSERT $value != NONE AND array::len($value) > 0;
DEFINE FIELD pickup_time ON TABLE order TYPE datetime
    ASSERT $value != NONE;
DEFINE FIELD status ON TABLE order TYPE string
    ASSERT $value INSIDE ["Pending", "Confirmed", "Preparing", "Ready", "PickedUp", "Cancelled"];
DEFINE FIELD total_amount ON TABLE order TYPE number
    ASSERT $value >= 0;
DEFINE FIELD created_at ON TABLE order TYPE datetime
    DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE order TYPE datetime
    DEFAULT time::now();

-- Indexes for order table
DEFINE INDEX order_number_idx ON TABLE order COLUMNS order_number UNIQUE;
DEFINE INDEX order_created_idx ON TABLE order COLUMNS created_at;
DEFINE INDEX order_pickup_idx ON TABLE order COLUMNS pickup_time;
DEFINE INDEX order_status_idx ON TABLE order COLUMNS status;
