use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Instant;

pub async fn log_request(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = Instant::now();

    tracing::info!(method = %method, uri = %uri, "Incoming request");

    let response = next.run(request).await;
    let duration = start.elapsed();

    tracing::info!(
        method = %method,
        uri = %uri,
        status = response.status().as_u16(),
        duration_ms = duration.as_millis(),
        "Request completed"
    );

    response
}
