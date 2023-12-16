use crate::{
    api::validation::validate_req_data,
    auth::user::get_user_id,
    db::{spaces::SpaceDb, DbPool},
    error::ServiceError,
};
use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicSpace;

// ======================================================================
// DTOs

#[derive(Deserialize, Validate)]
pub struct PaginationParams {
    #[validate(range(min = 1))]
    offset_id: Option<i32>,
    #[validate(range(min = 1))]
    limit: Option<i64>,
    #[validate(length(min = 16, max = 32))]
    building_id: Option<String>,
    user: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ListSpacesResponse {
    pub spaces: Vec<PublicSpace>,
    pub next_offset_id: Option<i32>,
    pub limit: i64,
}

// ======================================================================
// Route

#[get("")]
pub async fn list_spaces(
    req: actix_web::HttpRequest,
    pool: web::Data<DbPool>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, ServiceError> {
    let query = validate_req_data(query.into_inner())?;
    let user_id = get_user_id(&req)?;
    // default to user_id, but allow override
    let user = match query.user {
        // allow override
        Some(val) => match val {
            // if true, use user_id
            true => Some(user_id.clone()),
            false => None,
        },
        // default to user_id
        None => Some(user_id.clone()),
    };

    // limit default to 10, max 20
    let limit = query.limit.map_or(10, |l| if l > 20 { 20 } else { l });
    let spaces = SpaceDb::list(
        &pool,
        query.offset_id,
        limit,
        query.building_id.to_owned(),
        user,
    )
    .await?;
    let next_offset_id = if spaces.len() as i64 == limit {
        spaces.last().map(|b| b.id)
    } else {
        None
    };

    Ok(HttpResponse::Ok().json(ListSpacesResponse {
        spaces: spaces.into_iter().map(PublicSpace::from).collect(),
        next_offset_id,
        limit,
    }))
}
