use crate::{
    db::{spaces::SpaceDb, DbPool},
    error::ServiceError,
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
    space_id: web::Path<i32>,
) -> Result<HttpResponse, ServiceError> {
    let space = SpaceDb::get(&pool, space_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ReadSpaceResponse {
        space: space.into(),
    }))
}
