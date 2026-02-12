use axum::http::{header, HeaderValue, Method};
use tower_http::cors::{Any, CorsLayer};

pub fn create_cors_layer(allow_origin: &str) -> CorsLayer {
    // In development, allow multiple origins for Docker and localhost
    if allow_origin.contains("localhost") || allow_origin.contains("127.0.0.1") {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::OPTIONS,
                Method::PATCH,
            ])
            .allow_headers(Any)
            .allow_credentials(false)
    } else {
        // Production: strict CORS
        CorsLayer::new()
            .allow_origin(allow_origin.parse::<HeaderValue>().unwrap())
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::OPTIONS,
            ])
            .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
            .allow_credentials(false)
    }
}
