use actix_web::{post, HttpRequest, HttpResponse};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    auth::user::get_user_id,
    error::ServiceError,
    services::s3::{
        get_aws_region, get_credentials,
        presigned::{get_public_url, get_user_url},
    },
};

use super::S3_BUCKET_AVATAR_PATH;

// ======================================================================
// DTOs

#[derive(Debug, Serialize)]
pub struct PresignedUrlResponse {
    pub upload_url: String,
    pub fetch_url: String,
}

#[post("/avatar")]
pub async fn create_avatar_presigned_url(req: HttpRequest) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;

    let region = get_aws_region();
    let credentials = get_credentials();

    let path = format!("{}/{}", S3_BUCKET_AVATAR_PATH, Uuid::new_v4());
    let url = get_user_url(&region, &credentials, user_id, &path);

    Ok(HttpResponse::Ok().json(PresignedUrlResponse {
        upload_url: url.url,
        fetch_url: get_public_url(&get_aws_region(), &url.object_key),
    }))
}
