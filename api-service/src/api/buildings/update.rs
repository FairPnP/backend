use crate::{
    db::{buildings::BuildingDb, DbPool},
    error::ServiceError,
};
use actix_web::{put, web, HttpResponse};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use super::public::PublicBuilding;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize)]
pub struct UpdateBuildingRequest {
    pub name: Option<String>,
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
