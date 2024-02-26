use crate::{
    api::validation::validate_req_data,
    error::ServiceError,
    services::postgres::{spaces::images::SpaceImageDb, DbPool},
    utils::hashids::decode_id,
};
use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicSpaceImage;

// ======================================================================
// DTOs

#[derive(Deserialize, Validate)]
pub struct PaginationParams {
    #[validate(length(min = 10))]
    space_id: String,
}

#[derive(Debug, Serialize)]
pub struct ListSpaceImagesResponse {
    pub space_images: Vec<PublicSpaceImage>,
}

// ======================================================================
// Route

#[get("")]
pub async fn list_space_images(
    pool: web::Data<DbPool>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, ServiceError> {
    let query = validate_req_data(query.into_inner())?;
    let space_id = decode_id(&query.space_id)?;

    let space_images = SpaceImageDb::list_for_space(&pool, space_id).await?;

    Ok(HttpResponse::Ok().json(ListSpaceImagesResponse {
        space_images: space_images
            .into_iter()
            .map(PublicSpaceImage::from)
            .collect(),
    }))
}
