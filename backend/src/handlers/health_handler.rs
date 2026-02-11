use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

/// Health check endpoint for monitoring and Docker health checks
pub async fn health_check() -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "status": "healthy",
            "service": "royal-pizza-backend"
        })),
    )
}
