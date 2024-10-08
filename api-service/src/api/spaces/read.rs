use crate::{
    error::ServiceError,
    services::postgres::{
        spaces::{images::SpaceImageDb, SpaceDb},
        DbPool,
    },
    utils::hashids::decode_id,
};
use actix_web::{get, web, HttpResponse};
use serde::Serialize;

use super::public::PublicSpace;

// ======================================================================
// DTOs

#[derive(Debug, Serialize)]
pub struct ReadSpaceResponse {
    space: PublicSpace,
}

// ======================================================================
// Route

#[get("/{id}")]
pub async fn read_space(
    pool: web::Data<DbPool>,
    space_id: web::Path<String>,
) -> Result<HttpResponse, ServiceError> {
    let space_id = decode_id(&space_id.into_inner())?;
    let space = SpaceDb::get(&pool, space_id).await?;
    let mut public_space = PublicSpace::from(space);

    // get the space images
    let space_images = SpaceImageDb::list_for_space(&pool, space_id).await?;
    // populate the img_urls field
    public_space.img_urls = space_images
        .iter()
        .map(|img| img.img_url.to_owned())
        .collect::<Vec<String>>();

    Ok(HttpResponse::Ok().json(ReadSpaceResponse {
        space: public_space,
    }))
}
