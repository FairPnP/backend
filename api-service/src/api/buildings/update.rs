use crate::{
    api::validation::validate_req_data,
    db::{buildings::BuildingDb, DbPool},
    error::ServiceError,
};
use actix_web::{put, web, HttpResponse};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicBuilding;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateBuildingRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(min = 16, max = 32))]
    pub place_id: Option<String>,
    pub latitude: Option<BigDecimal>,
    pub longitude: Option<BigDecimal>,
}

#[derive(Debug, Serialize)]
pub struct UpdateBuildingResponse {
    building: PublicBuilding,
}

// ======================================================================
// Route

#[put("/{id}")]
pub async fn update_building(
    pool: web::Data<DbPool>,
    building_id: web::Path<i32>,
    data: web::Json<UpdateBuildingRequest>,
) -> Result<HttpResponse, ServiceError> {
    let data = validate_req_data(data.into_inner())?;
    let building_id = building_id.into_inner();

    let updated_building = BuildingDb::update(
        &pool,
        building_id,
        data.name.to_owned(),
        data.place_id.to_owned(),
        data.latitude.to_owned(),
        data.longitude.to_owned(),
    )
    .await?;
    Ok(HttpResponse::Ok().json(UpdateBuildingResponse {
        building: updated_building.into(),
    }))
}
