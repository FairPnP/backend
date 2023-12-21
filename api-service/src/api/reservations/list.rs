use crate::{
    api::validation::validate_req_data,
    auth::user::get_user_id,
    db::{reservations::ReservationDb, DbPool},
    error::ServiceError,
};
use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicReservation;

// ======================================================================
// DTOs

#[derive(Deserialize, Validate)]
pub struct PaginationParams {
    #[validate(range(min = 1))]
    offset_id: Option<i32>,
    #[validate(range(min = 1))]
    limit: Option<i64>,
    user: Option<bool>,
    #[validate(range(min = 1))]
    availability_id: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ListReservationsResponse {
    pub reservations: Vec<PublicReservation>,
    pub next_offset_id: Option<i32>,
    pub limit: i64,
}

// ======================================================================
// Route

#[get("")]
pub async fn list_reservations(
    req: actix_web::HttpRequest,
    pool: web::Data<DbPool>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let query = validate_req_data(query.into_inner())?;
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
    let reservations =
        ReservationDb::list(&pool, query.offset_id, limit, user, query.availability_id).await?;
    let next_offset_id = if reservations.len() as i64 == limit {
        reservations.last().map(|b| b.id)
    } else {
        None
    };

    Ok(HttpResponse::Ok().json(ListReservationsResponse {
        reservations: reservations
            .into_iter()
            .map(PublicReservation::from)
            .collect(),
        next_offset_id,
        limit,
    }))
}
