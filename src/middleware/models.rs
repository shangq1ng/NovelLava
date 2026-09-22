use axum::http::HeaderMap;
use governor::clock::DefaultClock;
use governor::middleware::NoOpMiddleware;
use governor::state::keyed::DefaultKeyedStateStore;
use governor::{Quota, RateLimiter};
use std::net::SocketAddr;
use std::num::NonZeroU32;
use std::sync::Arc;

pub type Limiter =
    Arc<RateLimiter<String, DefaultKeyedStateStore<String>, DefaultClock, NoOpMiddleware>>;
#[derive(Clone)]
pub struct RateLimit {
    pub(crate) limiter: Limiter,
}

impl RateLimit {
    pub async fn new(rq_per_sec: u32, burst_size: u32) -> Self {
        let quota = Quota::per_second(NonZeroU32::new(rq_per_sec).unwrap())
            .allow_burst(NonZeroU32::new(burst_size).unwrap());
        let limiter = Arc::new(RateLimiter::keyed(quota));
        let clone = Arc::clone(&limiter);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));
            loop {
                interval.tick().await;
                clone.retain_recent();
            }
        });
        Self { limiter }
    }
}

pub async fn extract_header(header: &HeaderMap, addr: SocketAddr) -> String {
    header
        .get("cf-connecting-ip") // cloudflare priority
        .or_else(|| header.get("x-real-ip"))
        .or_else(|| header.get("x-forwarded-for"))
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| addr.ip().to_string())
}
