use super::super::entities::{Building, PublicBuilding};
use crate::db::DbPool;
use crate::error::ServiceError;
use actix_web::{get, web, HttpResponse};
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
    let building = get_building_by_id(&pool, building_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ReadBuildingResponse {
        building: building.into(),
    }))
}

// ======================================================================
// Database operations

async fn get_building_by_id(pool: &DbPool, building_id: i32) -> Result<Building, ServiceError> {
    let building = sqlx::query_as::<_, Building>("SELECT * FROM buildings WHERE id = $1")
        .bind(building_id)
        .fetch_one(pool)
        .await?;
    Ok(building)
}
