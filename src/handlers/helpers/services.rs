use crate::models::errors::models::MiddlewareError;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_sessions::Session;
use uuid::Uuid;

#[derive(Clone, Copy)]
pub struct AuthenticatedUser(pub Uuid);
pub async fn check_if_logged_in(
    session: Session,
    mut req: Request,
    next: Next,
) -> Result<Response, MiddlewareError> {
    let id = session.get::<Uuid>("user_id").await?;
    let unset = "00000000-0000-0000-0000-000000000000";
    let nill_uuid = Uuid::parse_str(unset)?;

    match id {
        Some(id) => {
            if id != nill_uuid {
                req.extensions_mut().insert(AuthenticatedUser(id));
                Ok(next.run(req).await)
            } else {
                Ok(next.run(req).await)
            }
        }
        None => Ok(next.run(req).await),
    }
}
