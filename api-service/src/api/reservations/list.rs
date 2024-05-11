use crate::{
    api::validation::validate_req_data,
    auth::user::get_user_id,
    error::ServiceError,
    services::postgres::{
        reservations::{entities::ReservationStatus, ReservationDb},
        spaces::SpaceDb,
        DbPool,
    },
    utils::hashids::{decode_id, decode_id_option, encode_id},
};
use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicReservation;

// ======================================================================
// DTOs

#[derive(Deserialize, Validate)]
pub struct PaginationParams {
    #[validate(length(min = 10))]
    offset_id: Option<String>,
    #[validate(range(min = 1))]
    limit: Option<i32>,
    user: Option<bool>,
    #[validate(length(min = 10))]
    space_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListReservationsResponse {
    pub reservations: Vec<PublicReservation>,
    pub next_offset_id: Option<String>,
    pub limit: i32,
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
            true => Some(user_id),
            false => None,
        },
        // default to user_id
        None => Some(user_id),
    };

    if let Some(space_id) = query.space_id.clone() {
        if let Some(user) = &user {
            let space_id = decode_id(&space_id)?;
            let space = SpaceDb::get(&pool, space_id).await?;
            // if user is not the owner of the space, return unauthorized
            if space.user_id != *user {
                return Err(ServiceError::Unauthorized);
            }
        }
    }

    // limit default to 10, max 20
    let limit = query.limit.map_or(10, |l| if l > 20 { 20 } else { l });
    let offset_id = decode_id_option(&query.offset_id)?;
    let space_id = decode_id_option(&query.space_id)?;
    let reservations = ReservationDb::list(&pool, offset_id, limit, user, space_id, None).await?;
    let next_offset_id = if reservations.len() as i32 == limit {
        reservations.last().map(|b| encode_id(b.id))
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
