use crate::buildings::entities::{Building, NewBuilding, PublicBuilding};
use crate::db::{get_db_connection, DbPool};
use crate::error::ServiceError;
use crate::schema::buildings::dsl;
use actix_web::{post, web, HttpResponse};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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
    let new_building = NewBuilding {
        name: data.name.clone(),
        latitude: data.latitude.clone(),
        longitude: data.longitude.clone(),
        place_id: data.place_id.clone(),
    };

    let building = insert_new_building(&pool, new_building)?;
    Ok(HttpResponse::Created().json(CreateBuildingResponse {
        building: building.into(),
    }))
}

// ======================================================================
// Database operations

fn insert_new_building(pool: &DbPool, new_building: NewBuilding) -> Result<Building, ServiceError> {
    let mut conn = get_db_connection(pool)?;
    diesel::insert_into(dsl::buildings)
        .values(&new_building)
        .get_result(&mut conn)
        .map_err(From::from)
}
