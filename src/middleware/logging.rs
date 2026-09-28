use std::time::Instant;

use axum::{
    http::Request,
    body::Body,
    middleware::Next,
    response::Response
};

pub async fn request_logger(
    request: Request<Body>,
    next: Next
) -> Response {
    let method = request.method().clone();
    let path = request.uri().path().to_string();

    let start = Instant::now();

    let response = next.run(request).await;

    let latency = start.elapsed();

    tracing::debug!(
        method = %method,
        path = %path,
        status = %response.status(),
        latency_ms = latency.as_millis(),
        "Http Request"
    );

    response
}