use crate::middleware::models::{RateLimit, extract_header};
use crate::models::errors::models::MiddlewareError;
use axum::Json;
use axum::extract::{ConnectInfo, Request, State};
use axum::http::HeaderMap;
use axum::middleware::Next;
use axum::response::Response;
use governor::clock::Clock;
use serde_json::json;
use std::net::SocketAddr;

pub async fn rate_limiter(
    State(state): State<RateLimit>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    req: Request,
    next: Next,
) -> Result<Response, MiddlewareError> {
    let key = extract_header(&headers, addr).await;
    match state.limiter.check_key(&key) {
        Ok(()) => Ok(next.run(req).await),
        Err(negative_outcome) => {
            // Calculate remaining wait time
            let wait_time =
                negative_outcome.wait_time_from(governor::clock::DefaultClock::default().now());
            let _retry_after_secs = wait_time.as_secs().max(1);

            let body = Json(json!({
                "error": "Too Many Requests",
                "message": "Rate limit exceeded. Please slow down and try again after {retry_after_secs} seconds.",
            }));
            Err(MiddlewareError::TooManyRequests(body.to_string()))
        }
    }
}
