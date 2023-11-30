use super::super::entities::{Building, PublicBuilding, UpdateBuilding};
use crate::db::{get_db_connection, DbPool};
use crate::error::ServiceError;
use crate::schema::buildings::dsl;
use actix_web::{put, web, HttpResponse};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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
    let update_data = UpdateBuilding {
        name: data.name.clone(),
        place_id: data.place_id.clone(),
        latitude: data.latitude.clone(),
        longitude: data.longitude.clone(),
    };

    let updated_building = update_existing_building(&pool, building_id, update_data)?;
    Ok(HttpResponse::Ok().json(UpdateBuildingResponse {
        building: updated_building.into(),
    }))
}

// ======================================================================
// Database operations

fn update_existing_building(
    pool: &DbPool,
    building_id: i32,
    update_data: UpdateBuilding,
) -> Result<Building, ServiceError> {
    let mut conn = get_db_connection(pool)?;
    diesel::update(dsl::buildings.find(building_id))
        .set(&update_data)
        .get_result(&mut conn)
        .map_err(From::from)
}
