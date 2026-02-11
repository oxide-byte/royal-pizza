use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use shared::models::{CustomPizza, PizzaSize};

/// Cart item type - either standard or custom pizza
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CartItemType {
    StandardPizza {
        pizza_id: String,
        pizza_name: String,
        size: PizzaSize,
    },
    CustomPizza {
        custom: CustomPizza,
    },
}

impl CartItemType {
    /// Get display name for cart item
    pub fn display_name(&self) -> String {
        match self {
            CartItemType::StandardPizza { pizza_name, size, .. } => {
                format!("{} - {:?}", pizza_name, size)
            }
            CartItemType::CustomPizza { custom } => {
                format!("Custom Pizza - {:?}", custom.size)
            }
        }
    }
}

/// Individual cart item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CartItem {
    pub id: String,
    pub cart_item_type: CartItemType,
    pub quantity: u32,
    pub unit_price: f64,
}

impl CartItem {
    /// Calculate subtotal for this cart item
    pub fn subtotal(&self) -> f64 {
        self.quantity as f64 * self.unit_price
    }

    /// Create a unique ID for the cart item based on its type
    pub fn generate_id(cart_item_type: &CartItemType) -> String {
        match cart_item_type {
            CartItemType::StandardPizza { pizza_id, size, .. } => {
                format!("{}_{:?}", pizza_id, size)
            }
            CartItemType::CustomPizza { custom } => {
                // Use hash of instructions + size for custom pizzas
                format!("custom_{}_{:?}",
                    custom.instructions.len(),
                    custom.size)
            }
        }
    }
}

/// Cart state manager using reactive signals
#[derive(Debug, Clone, Copy)]
pub struct CartState {
    items: RwSignal<Vec<CartItem>>,
}

impl CartState {
    /// Create a new cart state
    pub fn new() -> Self {
        Self {
            items: create_rw_signal(Vec::new()),
        }
    }

    /// Add item to cart (or increment quantity if exists)
    pub fn add_item(&self, cart_item_type: CartItemType, quantity: u32, unit_price: f64) {
        self.items.update(|items: &mut Vec<CartItem>| {
            let id = CartItem::generate_id(&cart_item_type);

            // Check if item already exists
            if let Some(existing) = items.iter_mut().find(|item| item.id == id) {
                existing.quantity += quantity;
            } else {
                // Add new item
                items.push(CartItem {
                    id,
                    cart_item_type,
                    quantity,
                    unit_price,
                });
            }
        });
    }

    /// Remove item from cart
    pub fn remove_item(&self, id: &str) {
        self.items.update(|items: &mut Vec<CartItem>| {
            items.retain(|item| item.id != id);
        });
    }

    /// Update item quantity
    pub fn update_quantity(&self, id: &str, quantity: u32) {
        if quantity == 0 {
            self.remove_item(id);
            return;
        }

        self.items.update(|items: &mut Vec<CartItem>| {
            if let Some(item) = items.iter_mut().find(|item| item.id == id) {
                item.quantity = quantity;
            }
        });
    }

    /// Clear all items from cart
    pub fn clear(&self) {
        self.items.set(Vec::new());
    }

    /// Calculate total price of all items
    pub fn total(&self) -> f64 {
        self.items.with(|items: &Vec<CartItem>| {
            items.iter().map(|item: &CartItem| item.subtotal()).sum()
        })
    }

    /// Get total item count
    pub fn item_count(&self) -> u32 {
        self.items.with(|items: &Vec<CartItem>| {
            items.iter().map(|item: &CartItem| item.quantity).sum()
        })
    }

    /// Check if cart is empty
    pub fn is_empty(&self) -> bool {
        self.items.with(|items: &Vec<CartItem>| items.is_empty())
    }

    /// Get all items (reactive read)
    pub fn items(&self) -> Vec<CartItem> {
        self.items.get()
    }

    /// Get items as a signal
    pub fn items_signal(&self) -> RwSignal<Vec<CartItem>> {
        self.items
    }
}

impl Default for CartState {
    fn default() -> Self {
        Self::new()
    }
}

/// Provide cart state to the context
pub fn provide_cart_state() -> CartState {
    let cart = CartState::new();
    provide_context(cart);
    cart
}

/// Use cart state from context
pub fn use_cart() -> CartState {
    use_context::<CartState>()
        .expect("CartState should be provided in the app root")
}
