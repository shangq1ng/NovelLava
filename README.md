# NovelLava - Read your next adventure in speed
![Rust](https://img.shields.io/badge/rust-1.98.1%2B-orange?logo=rust)
![Axum](https://img.shields.io/badge/axum-0.8.9-black?logo=rust)
![SQLx](https://img.shields.io/badge/sqlx-0.9-blue)
![License](https://img.shields.io/badge/license-Apache%202.0-blue)

A backend for a novel-reading platform — auth, profiles, search,
image uploads, rate limiting, review, all built in Rust with Axum.

## What it does

- Email/username/password auth with Argon2 hashing
- Session-based auth with a resilient UUID middleware that
  falls back to IP when no session exists
- Tiered rate limiting (anon: 3 req/s, burst 10;
  logged-in: 10 req/s, burst 25) via tower-governor
- Profile image uploads: multipart → format detection via `image`
  → PNG conversion → S3 (OpenDAL) → URL stored in Postgres while the converted png is stored in the s3 storage.
- Full-text search with Meilisearch: pagination, filtering by
  author/novel/tag/genre/etc, Redis caches the paginated pages
- Docker Compose: Postgres, two Redis instances, LocalStack S3, Meilisearch

## Running it
`cp env.example .env`
`nano <Fill .env with your credentials>`
`docker compose up`
`cargo run --release --bin novellava`

## Demo

See `image/` for screenshots.

# LICENSE
Apache-2.0