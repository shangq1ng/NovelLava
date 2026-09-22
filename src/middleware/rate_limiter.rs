use crate::middleware::models::{RateLimit, extract_header};
use axum::Json;
use axum::extract::{ConnectInfo, Request, State};
use axum::http::{HeaderMap, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use governor::clock::Clock;
use serde_json::json;
use std::net::SocketAddr;

pub async fn rate_limiter(
    State(state): State<RateLimit>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    req: Request,
    next: Next,
) -> Result<Response, Response> {
    let key = extract_header(&headers, addr).await;
    match state.limiter.check_key(&key) {
        Ok(()) => Ok(next.run(req).await),
        Err(negative_outcome) => {
            // Calculate remaining wait time
            let wait_time =
                negative_outcome.wait_time_from(governor::clock::DefaultClock::default().now());
            let retry_after_secs = wait_time.as_secs().max(1);

            let body = Json(json!({
                "error": "Too Many Requests",
                "message": "Rate limit exceeded. Please slow down.",
            }));
            let response = (
                StatusCode::TOO_MANY_REQUESTS,
                [("Retry-After", retry_after_secs.to_string())],
                body,
            )
                .into_response();

            Err(response)
        }
    }
}
