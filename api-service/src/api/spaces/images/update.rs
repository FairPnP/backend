use crate::{
    api::validation::validate_req_data,
    auth::user::get_user_id,
    db::{space_images::SpaceImageDb, spaces::SpaceDb, DbPool},
    error::ServiceError,
};
use actix_web::{put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicSpaceImage;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateSpaceImageRequest {
    pub slot_id: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct UpdateSpaceImageResponse {
    pub space_image: PublicSpaceImage,
}

// ======================================================================
// Route

#[put("/{id}")]
pub async fn update_space_image(
    pool: web::Data<DbPool>,
    space_image_id: web::Path<i32>,
    req: actix_web::HttpRequest,
    data: web::Json<UpdateSpaceImageRequest>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let data = validate_req_data(data.into_inner())?;
    let space_image_id = space_image_id.into_inner();

    // Add authorization logic here if needed

    let updated_space_image =
        SpaceImageDb::update(&pool, space_image_id, data.slot_id, data.img_url).await?;
    Ok(HttpResponse::Ok().json(UpdateSpaceImageResponse {
        space_image: updated_space_image.into(), // Convert to PublicSpaceImage
    }))
}
