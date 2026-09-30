use crate::handlers::helpers::services::format_avatar;
use crate::models::config::models::AppConfig;
use crate::models::errors::models::S3ObjectError;
use axum::http::StatusCode;
use bytes::Bytes;
use uuid::Uuid;

pub async fn upload_avatar(
    state: &AppConfig,
    id: Uuid,
    avatar: Bytes,
) -> Result<StatusCode, S3ObjectError> {
    if let Ok(v) = format_avatar(avatar).await {
        let op = state.s3.operator.clone();
        let url = format!("novellava/avatars/{}.png", id);
        let mut writer = op.writer(url.as_str()).await?;
        writer.write(v).await?;
        writer.close().await?;
        Ok(StatusCode::CREATED)
    } else {
        Err(S3ObjectError::Failed)
    }
}
