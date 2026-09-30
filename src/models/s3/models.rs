use crate::models::errors::models::S3ObjectError;
use anyhow::Context;
use opendal::Operator;
use opendal::layers::LoggingLayer;
use opendal::services::S3;
use std::env;

#[derive(Debug, Clone)]
pub struct S3ObjectConfig {
    pub endpoint: String,
    pub region: String,
    pub key_id: String,
    pub application_key: String,
    pub bucket_name: String,
}

impl S3ObjectConfig {
    pub async fn new() -> Result<Self, S3ObjectError> {
        let endpoint = env::var("ENDPOINT").context("Missing ENDPOINT variable")?;
        let region = env::var("REGION").context("Missing REGION variable")?;
        let key_id = env::var("KEY_ID").context("Missing KEY_ID variable")?;
        let application_key =
            env::var("APPLICATION_KEY").context("Missing APPLICATION_KEY variable")?;
        let bucket_name = env::var("BUCKET_NAME").context("Missing BUCKET_NAME variable")?;

        Ok(Self {
            endpoint,
            region,
            key_id,
            application_key,
            bucket_name,
        })
    }
}

#[derive(Debug, Clone)]
pub struct S3BucketOperator {
    pub operator: Operator,
}

impl S3BucketOperator {
    pub async fn new() -> Result<Self, S3ObjectError> {
        let keys = S3ObjectConfig::new().await?;
        let builder = S3::default()
            .bucket(keys.bucket_name.as_str())
            .region(keys.region.as_str())
            .endpoint(keys.endpoint.as_str())
            .access_key_id(keys.key_id.as_str())
            .secret_access_key(keys.application_key.as_str());
        let op = Operator::new(builder)?.layer(LoggingLayer::default());
        Ok(Self { operator: op })
    }
}
