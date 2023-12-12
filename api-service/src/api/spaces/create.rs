use crate::{
    auth::user::get_user_id,
    db::{spaces::SpaceDb, DbPool},
    error::ServiceError,
};
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};

use super::public::PublicSpace;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize)]
pub struct CreateSpaceRequest {
    pub building_id: i32,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateSpaceResponse {
    pub space: PublicSpace,
}

// ======================================================================
// Route

#[post("")]
pub async fn create_space(
    req: actix_web::HttpRequest,
    pool: web::Data<DbPool>,
    data: web::Json<CreateSpaceRequest>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let data = data.into_inner();

    let space = SpaceDb::insert(&pool, data.building_id, user_id, data.name.to_owned()).await?;
    Ok(HttpResponse::Created().json(CreateSpaceResponse {
        space: space.into(),
    }))
}
