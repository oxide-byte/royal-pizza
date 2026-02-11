-- Royal Pizza Database Schema
-- SurrealDB v2.6.0 Initialization Script

-- Use namespace and database
USE NS royalpizza DB development;

-- ========================================
-- PIZZA TABLE SCHEMA
-- ========================================
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

-- Indexes for performance
DEFINE INDEX pizza_name_idx ON TABLE pizza COLUMNS name UNIQUE;

-- ========================================
-- ORDER TABLE SCHEMA
-- ========================================
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

-- Indexes for performance
DEFINE INDEX order_number_idx ON TABLE order COLUMNS order_number UNIQUE;
DEFINE INDEX order_created_idx ON TABLE order COLUMNS created_at;
DEFINE INDEX order_pickup_idx ON TABLE order COLUMNS pickup_time;

-- ========================================
-- SEED DATA: PIZZAS
-- ========================================

-- 1. Margherita
CREATE pizza:margherita SET
  id = "pizza-1",
  name = "Margherita",
  description = "Classic tomato, mozzarella, and fresh basil",
  ingredients = ["Tomato Sauce", "Mozzarella", "Fresh Basil", "Extra Virgin Olive Oil"],
  price = { small: 8.99, medium: 12.99, large: 15.99 },
  image_url = NONE,
  is_available = true;

-- 2. Pepperoni
CREATE pizza:pepperoni SET
  id = "pizza-2",
  name = "Pepperoni",
  description = "Loaded with pepperoni and mozzarella",
  ingredients = ["Tomato Sauce", "Mozzarella", "Pepperoni"],
  price = { small: 9.99, medium: 13.99, large: 16.99 },
  image_url = NONE,
  is_available = true;

-- 3. Hawaiian
CREATE pizza:hawaiian SET
  id = "pizza-3",
  name = "Hawaiian",
  description = "Ham, pineapple, and mozzarella",
  ingredients = ["Tomato Sauce", "Mozzarella", "Ham", "Pineapple"],
  price = { small: 10.99, medium: 14.99, large: 17.99 },
  image_url = NONE,
  is_available = true;

-- 4. Veggie Supreme
CREATE pizza:veggie SET
  id = "pizza-4",
  name = "Veggie Supreme",
  description = "Fresh vegetables on a bed of mozzarella",
  ingredients = ["Tomato Sauce", "Mozzarella", "Bell Peppers", "Mushrooms", "Onions", "Black Olives", "Tomatoes"],
  price = { small: 11.99, medium: 15.99, large: 18.99 },
  image_url = NONE,
  is_available = true;

-- 5. BBQ Chicken
CREATE pizza:bbq_chicken SET
  id = "pizza-5",
  name = "BBQ Chicken",
  description = "Grilled chicken with BBQ sauce and red onions",
  ingredients = ["BBQ Sauce", "Mozzarella", "Grilled Chicken", "Red Onions", "Cilantro"],
  price = { small: 12.99, medium: 16.99, large: 19.99 },
  image_url = NONE,
  is_available = true;

-- 6. Meat Lovers
CREATE pizza:meat_lovers SET
  id = "pizza-6",
  name = "Meat Lovers",
  description = "Loaded with pepperoni, sausage, bacon, and ham",
  ingredients = ["Tomato Sauce", "Mozzarella", "Pepperoni", "Italian Sausage", "Bacon", "Ham"],
  price = { small: 13.99, medium: 17.99, large: 20.99 },
  image_url = NONE,
  is_available = true;

-- 7. Supreme
CREATE pizza:supreme SET
  id = "pizza-7",
  name = "Supreme",
  description = "The ultimate combination of meats and vegetables",
  ingredients = ["Tomato Sauce", "Mozzarella", "Pepperoni", "Italian Sausage", "Bell Peppers", "Mushrooms", "Onions", "Black Olives"],
  price = { small: 13.99, medium: 17.99, large: 20.99 },
  image_url = NONE,
  is_available = true;

-- 8. Quattro Formaggi (Four Cheese)
CREATE pizza:quattro_formaggi SET
  id = "pizza-8",
  name = "Quattro Formaggi",
  description = "A blend of mozzarella, parmesan, gorgonzola, and ricotta",
  ingredients = ["White Sauce", "Mozzarella", "Parmesan", "Gorgonzola", "Ricotta"],
  price = { small: 12.99, medium: 16.99, large: 19.99 },
  image_url = NONE,
  is_available = true;

-- 9. Buffalo Chicken
CREATE pizza:buffalo_chicken SET
  id = "pizza-9",
  name = "Buffalo Chicken",
  description = "Spicy buffalo chicken with ranch drizzle",
  ingredients = ["Buffalo Sauce", "Mozzarella", "Chicken", "Red Onions", "Ranch Drizzle"],
  price = { small: 12.99, medium: 16.99, large: 19.99 },
  image_url = NONE,
  is_available = true;
