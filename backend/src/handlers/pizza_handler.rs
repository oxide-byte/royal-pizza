use crate::config::AppState;
use crate::middleware::error::AppError;
use crate::services::pizza_service;
use axum::{
    extract::{Path, State},
    Json,
};
use shared::dto::GetPizzasResponse;
use shared::models::Pizza;

pub async fn get_all_pizzas(
    State(state): State<AppState>,
) -> Result<Json<GetPizzasResponse>, AppError> {
    let response = pizza_service::get_all_available_pizzas(&state.db).await?;
    Ok(Json(response))
}

pub async fn get_pizza_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Pizza>, AppError> {
    let pizza = pizza_service::get_pizza_by_id(&state.db, &id).await?;
    Ok(Json(pizza))
}
