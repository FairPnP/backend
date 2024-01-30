use crate::{
    api::validation::validate_req_data,
    auth::user::get_user_id,
    db::{spaces::SpaceDb, DbPool},
    error::ServiceError,
};
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicSpace;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize, Validate)]
pub struct CreateSpaceRequest {
    #[validate(range(min = 1))]
    pub building_id: i32,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub description: Option<String>,
    #[validate(length(min = 1))]
    pub max_vehicle_size: String,
    #[validate(length(min = 1))]
    pub coverage: String,
    pub height_clearance_cm: Option<i32>,
    pub access_restrictions: Option<String>,
    pub parking_instructions: Option<String>,
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
    let data = validate_req_data(data.into_inner())?;

    let space = SpaceDb::insert(
        &pool,
        user_id,
        data.building_id,
        data.name.to_owned(),
        data.description,
        data.max_vehicle_size,
        data.coverage,
        data.height_clearance_cm,
        data.access_restrictions,
        data.parking_instructions,
    )
    .await?;
    Ok(HttpResponse::Created().json(CreateSpaceResponse {
        space: space.into(),
    }))
}
