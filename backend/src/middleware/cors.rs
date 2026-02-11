use axum::http::{header, HeaderValue, Method};
use tower_http::cors::CorsLayer;

pub fn create_cors_layer(allow_origin: &str) -> CorsLayer {
    CorsLayer::new()
        .allow_origin(allow_origin.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_credentials(false)
}
