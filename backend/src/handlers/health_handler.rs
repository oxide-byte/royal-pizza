use axum::{extract::State, http::StatusCode, Json};
use chrono::Utc;
use serde_json::{json, Value};

use crate::config::AppState;

/// Health check endpoint for monitoring and Docker health checks
///
/// This endpoint checks:
/// - Service availability
/// - Database connectivity
/// - Returns current timestamp
///
/// Used by Docker health checks and monitoring systems
pub async fn health_check(State(state): State<AppState>) -> (StatusCode, Json<Value>) {
    // Check database connectivity
    let db_status = check_database_health(&state).await;

    let overall_status = if db_status.is_connected {
        "healthy"
    } else {
        "unhealthy"
    };

    let status_code = if db_status.is_connected {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (
        status_code,
        Json(json!({
            "status": overall_status,
            "service": "royal-pizza-backend",
            "database": db_status.status,
            "timestamp": Utc::now().to_rfc3339()
        })),
    )
}

/// Database health check structure
struct DbHealth {
    is_connected: bool,
    status: String,
}

/// Checks if the database is accessible and responsive
async fn check_database_health(state: &AppState) -> DbHealth {
    // Try a simple query to verify database connectivity
    match state.db.query("SELECT * FROM pizza LIMIT 0").await {
        Ok(_) => DbHealth {
            is_connected: true,
            status: "connected".to_string(),
        },
        Err(e) => DbHealth {
            is_connected: false,
            status: format!("disconnected: {}", e),
        },
    }
}
