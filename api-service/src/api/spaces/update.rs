use crate::{
    db::{spaces::SpaceDb, DbPool},
    error::ServiceError,
};
use actix_web::{put, web, HttpResponse};
use serde::{Deserialize, Serialize};

use super::public::PublicSpace;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize)]
pub struct UpdateSpaceRequest {
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateSpaceResponse {
    space: PublicSpace,
}

// ======================================================================
// Route

#[put("/{id}")]
pub async fn update_space(
    pool: web::Data<DbPool>,
    space_id: web::Path<i32>,
    data: web::Json<UpdateSpaceRequest>,
) -> Result<HttpResponse, ServiceError> {
    let space_id = space_id.into_inner();

    let updated_space = SpaceDb::update(&pool, space_id, data.name.to_owned()).await?;
    Ok(HttpResponse::Ok().json(UpdateSpaceResponse {
        space: updated_space.into(),
    }))
}
