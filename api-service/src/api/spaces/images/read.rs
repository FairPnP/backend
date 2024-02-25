use crate::{
    services::postgres::{spaces::images::SpaceImageDb, DbPool},
    error::ServiceError,
};
use actix_web::{get, web, HttpResponse};
use serde::Serialize;

use super::public::PublicSpaceImage;

// ======================================================================
// DTOs

#[derive(Debug, Serialize)]
pub struct ReadSpaceImageResponse {
    space_image: PublicSpaceImage,
}

// ======================================================================
// Route

#[get("/{id}")]
pub async fn read_space_image(
    pool: web::Data<DbPool>,
    space_image_id: web::Path<i32>,
) -> Result<HttpResponse, ServiceError> {
    let space_image = SpaceImageDb::get(&pool, *space_image_id).await?;
    Ok(HttpResponse::Ok().json(ReadSpaceImageResponse {
        space_image: space_image.into(),
    }))
}
