use crate::{
    db::{buildings::BuildingDb, DbPool},
    error::ServiceError,
};
use actix_web::{post, web, HttpResponse};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use super::public::PublicBuilding;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize)]
pub struct CreateBuildingRequest {
    pub name: String,
    pub place_id: String,
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
