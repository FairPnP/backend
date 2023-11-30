use crate::api::buildings::entities::{Building, PublicBuilding};
use crate::db::DbPool;
use crate::error::ServiceError;
use actix_web::{post, web, HttpResponse};
use bigdecimal::BigDecimal;
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
    let building = insert_new_building(
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

// ======================================================================
// Database operations

pub async fn insert_new_building(
    pool: &DbPool,
    name: String,
    place_id: String,
    latitude: BigDecimal,
    longitude: BigDecimal,
) -> Result<Building, sqlx::Error> {
    let building = sqlx::query_as::<_, Building>(
        "INSERT INTO buildings (name, place_id, latitude, longitude) VALUES ($1, $2, $3, $4) RETURNING *")
        .bind(&name)
        .bind(&place_id)
        .bind(latitude)
        .bind(longitude)
        .fetch_one(pool)
        .await?.into();
    Ok(building)
}
