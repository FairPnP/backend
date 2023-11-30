use super::super::entities::{Building, PublicBuilding};
use crate::db::DbPool;
use crate::error::ServiceError;
use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};

// ======================================================================
// DTOs

#[derive(Deserialize)]
pub struct PaginationParams {
    offset_id: Option<i32>,
    limit: Option<i64>,
    place_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListBuildingsResponse {
    pub buildings: Vec<PublicBuilding>,
    pub next_offset_id: Option<i32>,
    pub limit: i64,
}

// ======================================================================
// Route

#[get("")]
pub async fn list_buildings(
    pool: web::Data<DbPool>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, ServiceError> {
    // limit default to 10, max 20
    let limit = query.limit.map_or(10, |l| if l > 20 { 20 } else { l });
    let buildings =
        get_buildings_by_team_id(&pool, query.offset_id, limit, query.place_id.clone()).await?;
    let next_offset_id = if buildings.len() as i64 == limit {
        buildings.last().map(|b| b.id)
    } else {
        None
    };

    Ok(HttpResponse::Ok().json(ListBuildingsResponse {
        buildings: buildings.into_iter().map(PublicBuilding::from).collect(),
        next_offset_id,
        limit,
    }))
}

// ======================================================================
// Database operations

async fn get_buildings_by_team_id(
    pool: &DbPool,
    offset_id: Option<i32>,
    limit: i64,
    place_id: Option<String>, // Add this parameter
) -> Result<Vec<Building>, ServiceError> {
    let mut query = String::from("SELECT * FROM buildings");

    let mut conditions = vec![];
    if let Some(ref pid) = place_id {
        conditions.push(format!("place_id = '{}'", pid));
    }
    if let Some(oid) = offset_id {
        conditions.push(format!("id > {}", oid));
    }

    if !conditions.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&conditions.join(" AND "));
    }

    query.push_str(&format!(" ORDER BY id ASC LIMIT {}", limit));

    let buildings = sqlx::query_as::<_, Building>(&query)
        .fetch_all(pool)
        .await
        .map_err(ServiceError::from)?;

    Ok(buildings)
}
