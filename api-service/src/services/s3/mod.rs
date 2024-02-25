use rusoto_core::{
    credential::{AwsCredentials, StaticProvider},
    Region,
};
use rusoto_s3::{HeadBucketRequest, S3Client, S3};
use std::env;

use crate::error::ServiceError;

pub mod presigned;

pub fn get_aws_region() -> Region {
    env::var("AWS_REGION")
        .expect("AWS_REGION must be set")
        .parse()
        .expect("AWS_REGION must be a valid region")
}

pub fn get_s3_bucket_name() -> String {
    env::var("S3_BUCKET_USER_CONTENT").expect("S3_BUCKET_USER_CONTENT must be set")
}

pub fn get_credentials() -> AwsCredentials {
    let access_key = env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID must be set");
    let secret_key = env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY must be set");

    AwsCredentials::new(access_key, secret_key, None, None)
}

pub fn get_s3_client() -> S3Client {
    let credentials = get_credentials();
    let provider = StaticProvider::from(credentials);
    S3Client::new_with(
        rusoto_core::request::HttpClient::new().expect("Failed to create HTTP client"),
        provider,
        get_aws_region(),
    )
}

pub async fn do_health_check(s3_client: &S3Client) -> Result<(), ServiceError> {
    let bucket_name = get_s3_bucket_name();
    let head_bucket_req = HeadBucketRequest {
        bucket: bucket_name.to_owned(),
        ..Default::default()
    };

    s3_client
        .head_bucket(head_bucket_req)
        .await
        .map_err(|e| ServiceError::InternalError(format!("Health check failed: {}", e)))?;

    Ok(())
}
