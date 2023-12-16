use crate::{
    api::validation::{list_param::param_list_i32, validate_req_data},
    db::{buildings::BuildingDb, DbPool},
    error::ServiceError,
};
use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicBuilding;

// ======================================================================
// DTOs

#[derive(Deserialize, Validate)]
pub struct PaginationParams {
    offset_id: Option<i32>,
    limit: Option<i64>,
    // TODO: validate place_id
    #[validate(length(min = 16, max = 32))]
    place_id: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "param_list_i32")]
    ids: Option<Vec<i32>>,
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
    let query = validate_req_data(query.into_inner())?;

    // limit default to 10, max 20
    let limit = query.limit.map_or(10, |l| if l > 20 { 20 } else { l });
    let buildings = BuildingDb::list(
        &pool,
        query.offset_id,
        limit,
        query.place_id.clone(),
        query.ids,
    )
    .await?;
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
