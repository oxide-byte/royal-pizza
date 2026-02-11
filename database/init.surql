-- Royal Pizza Database Initialization
-- Seed data for pizza menu
-- Version: 1.0
-- Date: 2026-02-11

-- ============================================================================
-- SEED PIZZA DATA
-- ============================================================================
-- Insert 9 standard pizzas with realistic data
-- This script is idempotent - uses INSERT with ON DUPLICATE KEY UPDATE logic via DELETE first

-- Clean existing pizzas (for development re-runs)
-- In production, comment this out or remove
DELETE pizza WHERE name IN [
    'Margherita',
    'Pepperoni',
    'Hawaiian',
    'Vegetarian',
    'BBQ Chicken',
    'Meat Lovers',
    'Four Cheese',
    'Spicy Italian',
    'Mediterranean'
];

-- 1. Margherita - Classic Italian
INSERT INTO pizza {
    id: 'pizza-margherita',
    name: 'Margherita',
    description: 'Classic Italian pizza with fresh tomato sauce, mozzarella, and fresh basil leaves',
    ingredients: ['Tomato Sauce', 'Fresh Mozzarella', 'Fresh Basil', 'Olive Oil', 'Sea Salt'],
    price: {
        small: 10.99,
        medium: 14.99,
        large: 18.99
    },
    image_url: '/images/pizzas/margherita.jpg',
    is_available: true,
    created_at: time::now(),
    updated_at: time::now()
};

-- 2. Pepperoni - American Classic
INSERT INTO pizza {
    id: 'pizza-pepperoni',
    name: 'Pepperoni',
    description: 'Loaded with premium pepperoni slices and melted mozzarella cheese',
    ingredients: ['Tomato Sauce', 'Mozzarella', 'Pepperoni', 'Italian Herbs'],
    price: {
        small: 11.99,
        medium: 15.99,
        large: 19.99
    },
    image_url: '/images/pizzas/pepperoni.jpg',
    is_available: true,
    created_at: time::now(),
    updated_at: time::now()
};

-- 3. Hawaiian - Sweet and Savory
INSERT INTO pizza {
    id: 'pizza-hawaiian',
    name: 'Hawaiian',
    description: 'Sweet pineapple chunks and savory ham with mozzarella on tomato base',
    ingredients: ['Tomato Sauce', 'Mozzarella', 'Ham', 'Pineapple'],
    price: {
        small: 12.99,
        medium: 16.99,
        large: 20.99
    },
    image_url: '/images/pizzas/hawaiian.jpg',
    is_available: true,
    created_at: time::now(),
    updated_at: time::now()
};

-- 4. Vegetarian - Garden Fresh
INSERT INTO pizza {
    id: 'pizza-vegetarian',
    name: 'Vegetarian',
    description: 'Fresh mixed vegetables including bell peppers, mushrooms, onions, olives, and tomatoes',
    ingredients: ['Tomato Sauce', 'Mozzarella', 'Bell Peppers', 'Mushrooms', 'Red Onions', 'Black Olives', 'Cherry Tomatoes'],
    price: {
        small: 11.99,
        medium: 15.99,
        large: 19.99
    },
    image_url: '/images/pizzas/vegetarian.jpg',
    is_available: true,
    created_at: time::now(),
    updated_at: time::now()
};

-- 5. BBQ Chicken - Smoky and Sweet
INSERT INTO pizza {
    id: 'pizza-bbq-chicken',
    name: 'BBQ Chicken',
    description: 'Grilled chicken breast with tangy BBQ sauce, red onions, and cilantro',
    ingredients: ['BBQ Sauce', 'Mozzarella', 'Grilled Chicken', 'Red Onions', 'Cilantro'],
    price: {
        small: 13.99,
        medium: 17.99,
        large: 21.99
    },
    image_url: '/images/pizzas/bbq-chicken.jpg',
    is_available: true,
    created_at: time::now(),
    updated_at: time::now()
};

-- 6. Meat Lovers - Carnivore's Dream
INSERT INTO pizza {
    id: 'pizza-meat-lovers',
    name: 'Meat Lovers',
    description: 'Loaded with pepperoni, Italian sausage, ham, bacon, and ground beef',
    ingredients: ['Tomato Sauce', 'Mozzarella', 'Pepperoni', 'Italian Sausage', 'Ham', 'Bacon', 'Ground Beef'],
    price: {
        small: 14.99,
        medium: 18.99,
        large: 22.99
    },
    image_url: '/images/pizzas/meat-lovers.jpg',
    is_available: true,
    created_at: time::now(),
    updated_at: time::now()
};

-- 7. Four Cheese - Cheese Lover's Paradise
INSERT INTO pizza {
    id: 'pizza-four-cheese',
    name: 'Four Cheese',
    description: 'Artisan blend of mozzarella, parmesan, gorgonzola, and fontina cheeses',
    ingredients: ['White Sauce', 'Mozzarella', 'Parmesan', 'Gorgonzola', 'Fontina', 'Italian Herbs'],
    price: {
        small: 13.99,
        medium: 17.99,
        large: 21.99
    },
    image_url: '/images/pizzas/four-cheese.jpg',
    is_available: true,
    created_at: time::now(),
    updated_at: time::now()
};

-- 8. Spicy Italian - Heat and Flavor
INSERT INTO pizza {
    id: 'pizza-spicy-italian',
    name: 'Spicy Italian',
    description: 'Spicy Italian sausage with hot peppers, jalapeños, and crushed red pepper',
    ingredients: ['Tomato Sauce', 'Mozzarella', 'Spicy Italian Sausage', 'Hot Peppers', 'Jalapeños', 'Red Pepper Flakes'],
    price: {
        small: 12.99,
        medium: 16.99,
        large: 20.99
    },
    image_url: '/images/pizzas/spicy-italian.jpg',
    is_available: true,
    created_at: time::now(),
    updated_at: time::now()
};

-- 9. Mediterranean - Greek Inspired
INSERT INTO pizza {
    id: 'pizza-mediterranean',
    name: 'Mediterranean',
    description: 'Feta cheese, kalamata olives, sun-dried tomatoes, red onions, and fresh oregano',
    ingredients: ['Tomato Sauce', 'Mozzarella', 'Feta Cheese', 'Kalamata Olives', 'Sun-Dried Tomatoes', 'Red Onions', 'Fresh Oregano'],
    price: {
        small: 13.99,
        medium: 17.99,
        large: 21.99
    },
    image_url: '/images/pizzas/mediterranean.jpg',
    is_available: true,
    created_at: time::now(),
    updated_at: time::now()
};

-- Verify inserted data
SELECT count() as pizza_count FROM pizza GROUP ALL;
SELECT name, price FROM pizza ORDER BY name;
