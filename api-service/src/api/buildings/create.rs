use crate::{
    api::validation::validate_req_data,
    db::{buildings::BuildingDb, DbPool},
    error::ServiceError,
};
use actix_web::{post, web, HttpResponse};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicBuilding;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize, Validate)]
pub struct CreateBuildingRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    // TODO: validate place_id
    #[validate(length(min = 16, max = 32))]
    pub place_id: String,
    // TODO: validate latitude and longitude
    pub latitude: BigDecimal,
    pub longitude: BigDecimal,
}

#[derive(Debug, Serialize)]
pub struct CreateBuildingResponse {
    pub building: PublicBuilding,
}

// ======================================================================
// Route

#[post("")]
pub async fn create_building(
    pool: web::Data<DbPool>,
    data: web::Json<CreateBuildingRequest>,
) -> Result<HttpResponse, ServiceError> {
    let data = validate_req_data(data.into_inner())?;

    let building = BuildingDb::insert(
        pool.get_ref(),
        data.name.to_owned(),
        data.place_id.to_owned(),
        data.latitude.to_owned(),
        data.longitude.to_owned(),
    )
    .await?;
    Ok(HttpResponse::Created().json(CreateBuildingResponse {
        building: building.into(),
    }))
}
