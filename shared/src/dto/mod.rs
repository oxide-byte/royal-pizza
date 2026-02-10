use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::{CustomerInfo, OrderItemType, Pizza};

/// Request DTO for creating a new order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub customer: CustomerInfo,
    pub items: Vec<OrderItemRequest>,
    pub pickup_time: DateTime<Utc>,
}

/// Request DTO for an individual order item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItemRequest {
    pub item_type: OrderItemType,
    pub quantity: u32,
}

/// Response DTO after creating an order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderResponse {
    pub order_id: String,
    pub order_number: String,
    pub total_amount: f64,
    pub pickup_time: DateTime<Utc>,
}

/// Response DTO for fetching pizzas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPizzasResponse {
    pub pizzas: Vec<Pizza>,
}

/// Standard error response DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<String>>,
}

impl ErrorResponse {
    /// Create a simple error response
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            details: None,
        }
    }

    /// Create an error response with details
    pub fn with_details(error: impl Into<String>, details: Vec<String>) -> Self {
        Self {
            error: error.into(),
            details: Some(details),
        }
    }
}
