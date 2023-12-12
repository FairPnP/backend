use crate::{
    db::{buildings::BuildingDb, DbPool},
    error::ServiceError,
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
    building_id: web::Path<i32>,
) -> Result<HttpResponse, ServiceError> {
    let building = BuildingDb::get(&pool, building_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ReadBuildingResponse {
        building: building.into(),
    }))
}
