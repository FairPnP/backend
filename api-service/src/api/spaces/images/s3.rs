use actix_web::{post, web, HttpRequest, HttpResponse};
use rusoto_s3::{
    util::{PreSignedRequest, PreSignedRequestOption},
    PutObjectRequest,
};
use serde::{Deserialize, Serialize};
use std::env;
use validator::Validate;

use crate::{
    auth::user::get_user_id,
    db::{
        s3::{get_aws_region, get_credentials},
        space_images::{entities::SpaceImageStatus, SpaceImageDb},
        spaces::SpaceDb,
        DbPool,
    },
    error::ServiceError,
};

// ======================================================================
// DTOs

#[derive(Debug, Deserialize, Validate)]
pub struct CreateSpaceImageRequest {
    #[validate(range(min = 1))]
    pub space_id: i32,
    #[validate(range(min = 1, max = 5))]
    pub num_images: i32,
}

#[derive(Debug, Serialize)]
pub struct PendingSpaceImage {
    pub space_image_id: i32,
    pub slot_id: i32,
    pub presigned_url: String,
}

#[derive(Debug, Serialize)]
pub struct PresignedUrlResponse {
    pub url: Vec<PendingSpaceImage>,
}

#[post("")]
pub async fn create_space_image(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    data: web::Json<CreateSpaceImageRequest>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let space_id = data.space_id;
    let num_new_images = data.num_images;

    let space = SpaceDb::get(&pool, space_id).await?;
    // if user is not the owner of the space, return unauthorized
    if space.user_id != user_id {
        return Err(ServiceError::Unauthorized);
    }

    // get images
    let space_images = SpaceImageDb::list(&pool, space_id).await?;

    // make sure there will be 5 or less images
    if space_images.len() as i32 + num_new_images > 5 {
        return Err(ServiceError::BadRequest(
            "Cannot have more than 5 images".to_string(),
        ));
    }

    let region = get_aws_region();
    let credentials = get_credentials();

    let mut presigned_urls = Vec::with_capacity(num_new_images as usize);
    let bucket = env::var("S3_BUCKET_USER_CONTENT").expect("S3_BUCKET_USER_CONTENT must be set");

    for _i in 0..num_new_images {
        let uuid = uuid::Uuid::new_v4();
        let object_key = format!(
            "user-uploads/{}/spaces/{}/images/{}.jpg",
            user_id, space_id, uuid
        );

        let options = PreSignedRequestOption {
            expires_in: std::time::Duration::from_secs(60),
        };
        let req = PutObjectRequest {
            bucket: bucket.clone(),
            key: object_key.clone(),
            ..Default::default()
        };

        let url = req.get_presigned_url(&region, &credentials, &options);
        presigned_urls.push((object_key, url));
    }

    // create pending space images
    let mut pending_images = Vec::new();
    let num_images = space_images.len() as i32 + num_new_images;
    let mut current_url = 0;
    for i in 0..num_images {
        let slot_id = i;
        if space_images
            .iter()
            .find(|&x| x.slot_id == slot_id)
            .is_none()
        {
            let presigned_url = &presigned_urls[current_url];
            let public_url = format!(
                "https://{}.s3.{}.amazonaws.com/{}",
                bucket,
                region.name(),
                presigned_url.0,
            );

            let space_image = SpaceImageDb::insert(
                &pool,
                space_id,
                slot_id,
                public_url,
                SpaceImageStatus::Pending,
            )
            .await?;

            pending_images.push(PendingSpaceImage {
                space_image_id: space_image.id,
                slot_id,
                presigned_url: presigned_url.1.clone(),
            });

            current_url += 1;
        }
    }

    Ok(HttpResponse::Ok().json(PresignedUrlResponse {
        url: pending_images,
    }))
}
