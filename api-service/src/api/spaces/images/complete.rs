use crate::{
    api::validation::validate_req_data,
    auth::user::get_user_id,
    db::{
        spaces::images::{entities::SpaceImageStatus, SpaceImageDb},
        spaces::SpaceDb,
        DbPool,
    },
    error::ServiceError,
};
use actix_web::{put, web, HttpResponse};
use serde::Deserialize;
use validator::Validate;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateSpaceImageRequest {
    #[validate(length(min = 1))]
    pub space_image_ids: Vec<i32>,
}

// ======================================================================
// Route

#[put("images/complete")]
pub async fn complete_space_image(
    pool: web::Data<DbPool>,
    req: actix_web::HttpRequest,
    data: web::Json<UpdateSpaceImageRequest>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let data = validate_req_data(data.into_inner())?;
    let space_image_ids = data.space_image_ids;

    // list pending space images
    let space_image =
        SpaceImageDb::list(&pool, &space_image_ids, SpaceImageStatus::Pending).await?;
    // make sure all pending images exist
    if space_image.len() != space_image_ids.len() {
        return Err(ServiceError::BadRequest("invalid space image ids".into()));
    }
    // make sure images are for the same space
    let space_id = space_image[0].space_id;
    for image in &space_image {
        if image.space_id != space_id {
            return Err(ServiceError::BadRequest(
                "Space images must be for the same space".into(),
            ));
        }
    }

    // get space
    let space = SpaceDb::get(&pool, space_id).await?;
    // if user is not the owner of the space, return unauthorized
    if space.user_id != user_id {
        return Err(ServiceError::Unauthorized);
    }

    SpaceImageDb::approve_many(&pool, &space_image_ids).await?;
    Ok(HttpResponse::Ok().finish())
}
