use rusoto_core::{credential::AwsCredentials, Region};
use rusoto_s3::{
    util::{PreSignedRequest, PreSignedRequestOption},
    PutObjectRequest,
};
use uuid::Uuid;

use super::get_aws_region;

pub struct PreSignedUrl {
    pub object_key: String,
    pub url: String,
}

pub fn get_user_url(
    region: &Region,
    credentials: &AwsCredentials,
    user_id: Uuid,
    path: &str,
) -> PreSignedUrl {
    let bucket =
        std::env::var("S3_BUCKET_USER_CONTENT").expect("S3_BUCKET_USER_CONTENT must be set");
    let object_key = format!("user-uploads/{}/{}", user_id, path);

    let options = PreSignedRequestOption {
        expires_in: std::time::Duration::from_secs(60),
    };
    let req = PutObjectRequest {
        bucket: bucket.into(),
        key: object_key.clone(),
        ..Default::default()
    };

    let url = req.get_presigned_url(&region, &credentials, &options);
    PreSignedUrl { object_key, url }
}

pub fn get_public_url(region: &Region, object_key: &str) -> String {
    let bucket =
        std::env::var("S3_BUCKET_USER_CONTENT").expect("S3_BUCKET_USER_CONTENT must be set");
    format!(
        "https://{}.s3.{}.amazonaws.com/{}",
        bucket,
        region.name(),
        object_key
    )
}

pub fn is_valid_url(url: &str, user_id: Uuid, path_prefix: &str) -> bool {
    let region = get_aws_region();
    let bucket =
        std::env::var("S3_BUCKET_USER_CONTENT").expect("S3_BUCKET_USER_CONTENT must be set");
    let prefix = format!(
        "https://{}.s3.{}.amazonaws.com/user-uploads/{}/{}",
        bucket,
        region.name(),
        user_id,
        path_prefix
    );

    url.starts_with(&prefix)
}
