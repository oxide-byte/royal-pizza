use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use shared::dto::ErrorResponse;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    ValidationError(Vec<String>),
    DatabaseError(String),
    InternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_response) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, ErrorResponse::new(msg)),
            AppError::ValidationError(errors) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ErrorResponse::with_details("Validation failed".to_string(), errors),
            ),
            AppError::DatabaseError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse::new(format!("Database error: {}", msg)),
            ),
            AppError::InternalError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse::new(format!("Internal error: {}", msg)),
            ),
        };

        (status, Json(error_response)).into_response()
    }
}

impl From<surrealdb::Error> for AppError {
    fn from(err: surrealdb::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}
