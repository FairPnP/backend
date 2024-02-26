use crate::{
    api::validation::validate_req_data,
    auth::user::get_user_id,
    error::ServiceError,
    services::postgres::{availability::AvailabilityDb, DbPool},
    utils::hashids::decode_id_option,
};
use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicAvailability;

// ======================================================================
// DTOs

#[derive(Deserialize, Validate)]
pub struct PaginationParams {
    #[validate(range(min = 1))]
    offset_id: Option<i32>,
    #[validate(range(min = 1))]
    limit: Option<i32>,
    user: Option<bool>,
    #[validate(length(min = 10))]
    space_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListAvailabilityResponse {
    pub availability: Vec<PublicAvailability>,
    pub next_offset_id: Option<i32>,
    pub limit: i32,
}

// ======================================================================
// Route

#[get("")]
pub async fn list_availability(
    req: actix_web::HttpRequest,
    pool: web::Data<DbPool>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, ServiceError> {
    let query = validate_req_data(query.into_inner())?;
    let user_id = get_user_id(&req)?;
    let space_id = decode_id_option(&query.space_id)?;
    // default to user_id, but allow override
    let user = match query.user {
        // allow override
        Some(val) => match val {
            // if true, use user_id
            true => Some(user_id),
            false => None,
        },
        // default to user_id
        None => Some(user_id),
    };

    // limit default to 10, max 20
    let limit = query.limit.map_or(10, |l| if l > 20 { 20 } else { l });
    let availability = AvailabilityDb::list(&pool, query.offset_id, limit, user, space_id).await?;
    let next_offset_id = if availability.len() as i32 == limit {
        availability.last().map(|b| b.id)
    } else {
        None
    };

    Ok(HttpResponse::Ok().json(ListAvailabilityResponse {
        availability: availability
            .into_iter()
            .map(PublicAvailability::from)
            .collect(),
        next_offset_id,
        limit,
    }))
}
