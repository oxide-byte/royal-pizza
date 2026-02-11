use serde::{Deserialize, Serialize};

/// Represents a pizza in the menu
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

/// Price structure for different pizza sizes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PizzaPrice {
    pub small: f64,
    pub medium: f64,
    pub large: f64,
}

/// Pizza size enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PizzaSize {
    Small,
    Medium,
    Large,
}

impl PizzaSize {
    /// Get the price for this size from a PizzaPrice struct
    pub fn get_price(&self, price: &PizzaPrice) -> f64 {
        match self {
            PizzaSize::Small => price.small,
            PizzaSize::Medium => price.medium,
            PizzaSize::Large => price.large,
        }
    }
}

/// Custom pizza order with special instructions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomPizza {
    pub instructions: String,
    pub size: PizzaSize,
}

impl CustomPizza {
    /// Fixed pricing for custom pizzas
    pub fn get_price(&self) -> f64 {
        match self.size {
            PizzaSize::Small => 10.99,
            PizzaSize::Medium => 14.99,
            PizzaSize::Large => 17.99,
        }
    }
}
