use crate::{
    api::validation::{list_param::param_list_i32, validate_req_data},
    error::ServiceError,
    services::postgres::{buildings::BuildingDb, DbPool},
    utils::hashids::{decode_id_option, encode_id},
};
use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicBuilding;

// ======================================================================
// DTOs

#[derive(Deserialize, Validate)]
pub struct PaginationParams {
    #[validate(length(min = 10))]
    offset_id: Option<String>,
    limit: Option<i32>,
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
    pub next_offset_id: Option<String>,
    pub limit: i32,
}

// ======================================================================
// Route

#[get("")]
pub async fn list_buildings(
    pool: web::Data<DbPool>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, ServiceError> {
    let query = validate_req_data(query.into_inner())?;
    let offset_id = decode_id_option(&query.offset_id)?;

    // limit default to 10, max 20
    let limit = query.limit.map_or(10, |l| if l > 20 { 20 } else { l });
    let buildings =
        BuildingDb::list(&pool, offset_id, limit, query.place_id.clone(), query.ids).await?;
    let next_offset_id = if buildings.len() as i32 == limit {
        buildings.last().map(|b| encode_id(b.id))
    } else {
        None
    };

    Ok(HttpResponse::Ok().json(ListBuildingsResponse {
        buildings: buildings.into_iter().map(PublicBuilding::from).collect(),
        next_offset_id,
        limit,
    }))
}
