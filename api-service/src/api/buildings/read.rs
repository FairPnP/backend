use crate::{
    error::ServiceError,
    services::postgres::{buildings::BuildingDb, DbPool},
    utils::hashids::decode_id,
};
use actix_web::{get, web, HttpResponse};
use serde::Serialize;

use super::public::PublicBuilding;

// ======================================================================
// DTOs

#[derive(Debug, Serialize)]
pub struct ReadBuildingResponse {
    building: PublicBuilding,
}

// ======================================================================
// Route

#[get("/{id}")]
pub async fn read_building(
    pool: web::Data<DbPool>,
    building_id: web::Path<String>,
) -> Result<HttpResponse, ServiceError> {
    let building_id = decode_id(&building_id.into_inner())?;
    let building = BuildingDb::get(&pool, building_id).await?;
    Ok(HttpResponse::Ok().json(ReadBuildingResponse {
        building: building.into(),
    }))
}
