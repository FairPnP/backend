// reservations/list_host_reservations.rs
use crate::{
    api::validation::validate_req_data,
    auth::user::get_user_id,
    error::ServiceError,
    services::postgres::{reservations::ReservationDb, DbPool},
    utils::hashids::{decode_id_option, encode_id},
};
use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicReservation;

// ======================================================================
// DTOs

#[derive(Deserialize, Validate)]
pub struct HostPaginationParams {
    #[validate(length(min = 10))]
    offset_id: Option<String>,
    #[validate(range(min = 1))]
    limit: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ListHostReservationsResponse {
    pub reservations: Vec<PublicReservation>,
    pub next_offset_id: Option<String>,
    pub limit: i32,
}

// ======================================================================
// Route

#[get("/host")]
pub async fn list_host_reservations(
    req: actix_web::HttpRequest,
    pool: web::Data<DbPool>,
    query: web::Query<HostPaginationParams>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let query = validate_req_data(query.into_inner())?;

    // limit default to 10, max 20
    let limit = query.limit.map_or(10, |l| if l > 20 { 20 } else { l });
    let offset_id = decode_id_option(&query.offset_id)?;
    let reservations = ReservationDb::list_for_host(&pool, user_id, offset_id, limit).await?;
    let next_offset_id = if reservations.len() as i32 == limit {
        reservations.last().map(|b| encode_id(b.id))
    } else {
        None
    };

    Ok(HttpResponse::Ok().json(ListHostReservationsResponse {
        reservations: reservations
            .into_iter()
            .map(PublicReservation::from)
            .collect(),
        next_offset_id,
        limit,
    }))
}
