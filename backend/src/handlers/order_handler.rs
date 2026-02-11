use crate::config::AppState;
use crate::middleware::error::AppError;
use crate::services::order_service;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use shared::dto::{CreateOrderRequest, CreateOrderResponse};
use shared::models::Order;

pub async fn create_order_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateOrderRequest>,
) -> Result<(StatusCode, Json<CreateOrderResponse>), AppError> {
    let response = order_service::create_order(&state.db, request).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_order_by_id_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Order>, AppError> {
    let order = order_service::get_order_by_id(&state.db, &id).await?;
    Ok(Json(order))
}
