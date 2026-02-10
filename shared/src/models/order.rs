use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::pizza::{CustomPizza, PizzaSize};

/// Complete order information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub order_number: String,
    pub customer: CustomerInfo,
    pub items: Vec<OrderItem>,
    pub pickup_time: DateTime<Utc>,
    pub status: OrderStatus,
    pub total_amount: f64,
    pub created_at: DateTime<Utc>,
}

/// Customer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerInfo {
    pub name: String,
    pub phone: String,
}

/// Individual item in an order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub id: String,
    pub item_type: OrderItemType,
    pub quantity: u32,
    pub unit_price: f64,
    pub subtotal: f64,
}

impl OrderItem {
    /// Calculate the subtotal for this item
    pub fn calculate_subtotal(quantity: u32, unit_price: f64) -> f64 {
        quantity as f64 * unit_price
    }

    /// Create a new order item with calculated subtotal
    pub fn new(id: String, item_type: OrderItemType, quantity: u32, unit_price: f64) -> Self {
        let subtotal = Self::calculate_subtotal(quantity, unit_price);
        Self {
            id,
            item_type,
            quantity,
            unit_price,
            subtotal,
        }
    }
}

/// Type of pizza item (standard from menu or custom)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum OrderItemType {
    StandardPizza { pizza_id: String, size: PizzaSize },
    CustomPizza { custom: CustomPizza },
}

/// Order status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Preparing,
    Ready,
    PickedUp,
    Cancelled,
}

impl OrderStatus {
    /// Convert status to string for display
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderStatus::Pending => "Pending",
            OrderStatus::Confirmed => "Confirmed",
            OrderStatus::Preparing => "Preparing",
            OrderStatus::Ready => "Ready",
            OrderStatus::PickedUp => "Picked Up",
            OrderStatus::Cancelled => "Cancelled",
        }
    }
}
