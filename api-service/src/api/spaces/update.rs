use crate::{
    api::validation::validate_req_data,
    auth::user::get_user_id,
    db::{spaces::SpaceDb, DbPool},
    error::ServiceError,
};
use actix_web::{put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicSpace;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateSpaceRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    pub description: Option<String>,
    pub max_vehicle_size: Option<String>,
    pub coverage: Option<String>,
    pub height_clearance_cm: Option<Option<i32>>, // Double Option to allow setting the field to NULL
    pub access_restrictions: Option<String>,
    pub parking_instructions: Option<String>,
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
    req: actix_web::HttpRequest,
    data: web::Json<UpdateSpaceRequest>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let data = validate_req_data(data.into_inner())?;
    let space_id = space_id.into_inner();

    let updated_space = SpaceDb::update(
        &pool,
        space_id,
        user_id,
        data.name.to_owned(),
        data.description,
        data.max_vehicle_size,
        data.coverage,
        data.height_clearance_cm,
        data.access_restrictions,
        data.parking_instructions,
    )
    .await?;
    Ok(HttpResponse::Ok().json(UpdateSpaceResponse {
        space: updated_space.into(),
    }))
}
