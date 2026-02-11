use std::fmt;

/// API error types
#[derive(Debug, Clone)]
pub enum ApiError {
    NetworkError(String),
    HttpError { status: u16, message: String },
    ParseError(String),
    SerializeError(String),
}

impl ApiError {
    /// Convert error to user-friendly message
    pub fn user_message(&self) -> String {
        match self {
            ApiError::NetworkError(_) => {
                "Unable to connect to the server. Please check your internet connection and try again.".to_string()
            }
            ApiError::HttpError { status, message } => {
                match *status {
                    400 => format!("Invalid request: {}", message),
                    404 => "The requested resource was not found.".to_string(),
                    500..=599 => "Server error. Please try again later.".to_string(),
                    _ => format!("Request failed: {}", message),
                }
            }
            ApiError::ParseError(_) => {
                "Failed to process server response. Please try again.".to_string()
            }
            ApiError::SerializeError(_) => {
                "Failed to prepare request. Please check your input and try again.".to_string()
            }
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ApiError::HttpError { status, message } => {
                write!(f, "HTTP error {}: {}", status, message)
            }
            ApiError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ApiError::SerializeError(msg) => write!(f, "Serialize error: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}
