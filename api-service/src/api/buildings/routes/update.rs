use super::super::entities::{Building, PublicBuilding};
use crate::db::DbPool;
use crate::error::ServiceError;
use actix_web::{put, web, HttpResponse};
use bigdecimal::BigDecimal;
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

    let updated_building = update_existing_building(
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

// ======================================================================
// Database operations

async fn update_existing_building(
    pool: &DbPool,
    building_id: i32,
    name: Option<String>,
    place_id: Option<String>,
    latitude: Option<BigDecimal>,
    longitude: Option<BigDecimal>,
) -> Result<Building, ServiceError> {
    let building = sqlx::query_as::<_, Building>(
        "UPDATE buildings SET name = COALESCE($1, name), place_id = COALESCE($2, place_id), latitude = COALESCE($3, latitude), longitude = COALESCE($4, longitude) WHERE id = $5 RETURNING *")
        .bind(name)
        .bind(place_id)
        .bind(latitude)
        .bind(longitude)
        .bind(building_id)
        .fetch_one(pool)
        .await?;

    Ok(building)
}
