use crate::buildings::entities::{Building, PublicBuilding};
use crate::db::{get_db_connection, DbPool};
use crate::error::ServiceError;
use crate::schema::buildings::dsl;
use actix_web::{get, web, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// ======================================================================
// DTOs

#[derive(Deserialize)]
pub struct PaginationParams {
    offset_id: Option<i32>,
    limit: Option<i64>,
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
    let buildings = get_buildings_by_team_id(&pool, query.offset_id, limit)?;
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

fn get_buildings_by_team_id(
    pool: &DbPool,
    offset_id: Option<i32>,
    limit: i64,
) -> Result<Vec<Building>, ServiceError> {
    let mut conn = get_db_connection(pool)?;
    let mut query = dsl::buildings.order(dsl::id.asc()).into_boxed();

    if let Some(offset_id) = offset_id {
        query = query.filter(dsl::id.gt(offset_id));
    }

    query
        .limit(limit)
        .load::<Building>(&mut conn)
        .map_err(From::from)
}
