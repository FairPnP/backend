use super::super::entities::{Building, PublicBuilding};
use crate::db::{get_db_connection, DbPool};
use crate::error::ServiceError;
use crate::schema::buildings::dsl;
use actix_web::{get, web, HttpResponse};
use diesel::prelude::*;
use serde::Serialize;

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
    let building = get_building_by_id(&pool, building_id.into_inner())?;
    Ok(HttpResponse::Ok().json(ReadBuildingResponse {
        building: building.into(),
    }))
}

// ======================================================================
// Database operations

fn get_building_by_id(pool: &DbPool, building_id: i32) -> Result<Building, ServiceError> {
    let mut conn = get_db_connection(pool)?;
    dsl::buildings
        .find(building_id)
        .get_result(&mut conn)
        .map_err(From::from)
}
