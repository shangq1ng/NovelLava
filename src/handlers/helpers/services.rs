use crate::models::errors::models::MiddlewareError;
use axum::extract::{ConnectInfo, Request};
use axum::middleware::Next;
use axum::response::Response;
use bytes::Bytes;
use std::net::{IpAddr, SocketAddr};
use tower_sessions::Session;
use uuid::Uuid;

#[derive(Clone, Copy)]
pub struct AuthenticatedUser(pub Uuid);
#[derive(Clone, Copy)]
pub struct AnonymousUser(pub IpAddr);
pub async fn check_if_logged_in(
    session: Session,
    mut req: Request,
    next: Next,
) -> Result<Response, MiddlewareError> {
    let addr = *req
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .ok_or(MiddlewareError::NotFound)?;

    let id = session.get::<Uuid>("user_id").await?;
    let nill = Uuid::nil();
    #[allow(clippy::collapsible_match)]
    match id {
        Some(id) => {
            if id != nill {
                req.extensions_mut().insert(AuthenticatedUser(id));
                Ok(next.run(req).await)
            } else {
                req.extensions_mut().insert(AnonymousUser(addr.ip()));
                Ok(next.run(req).await)
            }
        }
        None => {
            req.extensions_mut().insert(AnonymousUser(addr.ip()));
            Ok(next.run(req).await)
        }
    }
}

pub async fn format_avatar(avatar: Bytes) -> Result<Vec<u8>, image::ImageError> {
    let img = image::load_from_memory(&avatar);
    let img = img?.resize_to_fill(256, 256, image::imageops::FilterType::Lanczos3);
    let mut buf = std::io::Cursor::new(Vec::new());
    img.write_to(&mut buf, image::ImageFormat::Png)?;
    Ok(buf.into_inner())
}
