use crate::config::AppState;
use crate::handlers::{health_handler, order_handler, pizza_handler};
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        // Health check
        .route("/health", get(health_handler::health_check))
        // Pizza routes
        .route("/pizzas", get(pizza_handler::get_all_pizzas))
        .route("/pizzas/:id", get(pizza_handler::get_pizza_by_id))
        // Order routes
        .route("/orders", post(order_handler::create_order_handler))
        .route("/orders/:id", get(order_handler::get_order_by_id_handler))
}
