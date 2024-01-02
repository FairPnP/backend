use actix_web::{get, web, HttpRequest, HttpResponse};
use rusoto_s3::{
    util::{PreSignedRequest, PreSignedRequestOption},
    PutObjectRequest,
};
use serde::Serialize;
use std::env;

use crate::{
    auth::user::get_user_id,
    db::{
        s3::{get_aws_region, get_credentials},
        spaces::SpaceDb,
        DbPool,
    },
    error::ServiceError,
};

// ======================================================================
// DTOs

#[derive(Debug, Serialize)]
pub struct PresignedUrlResponse {
    pub url: String,
}

#[get("/{space_id}/presigned-url")]
pub async fn generate_presigned_url(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    space_id: web::Path<i32>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let space_id = space_id.into_inner();

    // get space from db
    let space = SpaceDb::get(&pool, space_id).await?;

    // check if user is owner of space
    if space.user_id != user_id {
        return Err(ServiceError::Unauthorized);
    }

    let region = get_aws_region();
    let credentials = get_credentials();

    // Generate a unique key for the S3 object
    let object_key = format!("user-uploads/{}/space-images/{}.jpg", user_id, space_id);

    let options = PreSignedRequestOption {
        expires_in: std::time::Duration::from_secs(60),
    };
    let req = PutObjectRequest {
        bucket: env::var("S3_BUCKET_USER_CONTENT").expect("S3_BUCKET_USER_CONTENT must be set"),
        key: object_key,
        ..Default::default()
    };

    let url = req.get_presigned_url(&region, &credentials, &options);

    Ok(HttpResponse::Ok().json(PresignedUrlResponse { url }))
}
