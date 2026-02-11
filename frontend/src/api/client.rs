use gloo_net::http::Request;
use shared::dto::{CreateOrderRequest, CreateOrderResponse, GetPizzasResponse};
use shared::models::{Order, Pizza};

use super::error::ApiError;

const API_BASE_URL: &str = match option_env!("TRUNK_API_BASE_URL") {
    Some(url) => url,
    None => "http://localhost:8080/api", // Development fallback
};

/// Fetch all pizzas from the API
pub async fn fetch_pizzas() -> Result<Vec<Pizza>, ApiError> {
    let url = format!("{}/pizzas", API_BASE_URL);

    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| ApiError::NetworkError(e.to_string()))?;

    if !response.ok() {
        let status = response.status();
        let message = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(ApiError::HttpError { status, message });
    }

    let data: GetPizzasResponse = response
        .json()
        .await
        .map_err(|e| ApiError::ParseError(e.to_string()))?;

    Ok(data.pizzas)
}

/// Fetch a single pizza by ID
pub async fn fetch_pizza_by_id(id: &str) -> Result<Pizza, ApiError> {
    let url = format!("{}/pizzas/{}", API_BASE_URL, id);

    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| ApiError::NetworkError(e.to_string()))?;

    if !response.ok() {
        let status = response.status();
        let message = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(ApiError::HttpError { status, message });
    }

    let pizza: Pizza = response
        .json()
        .await
        .map_err(|e| ApiError::ParseError(e.to_string()))?;

    Ok(pizza)
}

/// Create a new order
pub async fn create_order(
    request: CreateOrderRequest,
) -> Result<CreateOrderResponse, ApiError> {
    let url = format!("{}/orders", API_BASE_URL);

    let body = serde_json::to_string(&request)
        .map_err(|e| ApiError::SerializeError(e.to_string()))?;

    let response = Request::post(&url)
        .header("Content-Type", "application/json")
        .body(body)
        .map_err(|e| ApiError::SerializeError(e.to_string()))?
        .send()
        .await
        .map_err(|e| ApiError::NetworkError(e.to_string()))?;

    if !response.ok() {
        let status = response.status();
        let message = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(ApiError::HttpError { status, message });
    }

    let order_response: CreateOrderResponse = response
        .json()
        .await
        .map_err(|e| ApiError::ParseError(e.to_string()))?;

    Ok(order_response)
}

/// Fetch an order by ID
pub async fn fetch_order_by_id(id: &str) -> Result<Order, ApiError> {
    let url = format!("{}/orders/{}", API_BASE_URL, id);

    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| ApiError::NetworkError(e.to_string()))?;

    if !response.ok() {
        let status = response.status();
        let message = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(ApiError::HttpError { status, message });
    }

    let order: Order = response
        .json()
        .await
        .map_err(|e| ApiError::ParseError(e.to_string()))?;

    Ok(order)
}
