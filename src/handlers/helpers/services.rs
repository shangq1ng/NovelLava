use crate::handlers::auth::helpers::TemplateRender;
use crate::models::auth::models::HomePage;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_sessions::Session;
use uuid::Uuid;

#[derive(Clone, Copy)]
pub struct AuthenticatedUser(pub Uuid);
#[axum::debug_middleware]
pub async fn check_if_logged_in(session: Session, mut req: Request, next: Next) -> Response {
    if let Ok(user_id) = session.get::<Uuid>("user_id").await {
        req.extensions_mut()
            .insert(AuthenticatedUser(user_id.unwrap_or_default()));
    }
    next.run(req).await
}
pub async fn home() -> Response {
    HomePage { error_msg: None }.into_html()
}
