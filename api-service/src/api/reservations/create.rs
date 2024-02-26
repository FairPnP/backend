use crate::{
    api::validation::validate_req_data,
    auth::user::get_user_id,
    error::ServiceError,
    services::postgres::{reservations::ReservationDb, DbPool},
    utils::hashids::decode_id,
};
use actix_web::{post, web, HttpResponse};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicReservation;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize, Validate)]
pub struct CreateReservationRequest {
    #[validate(length(min = 10))]
    pub space_id: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct CreateReservationResponse {
    pub reservation: PublicReservation,
}

// ======================================================================
// Route

#[post("")]
pub async fn create_reservation(
    req: actix_web::HttpRequest,
    pool: web::Data<DbPool>,
    data: web::Json<CreateReservationRequest>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let data = validate_req_data(data.into_inner())?;
    let space_id = decode_id(&data.space_id)?;

    let reservation =
        ReservationDb::insert(&pool, user_id, space_id, data.start_date, data.end_date).await?;
    Ok(HttpResponse::Created().json(CreateReservationResponse {
        reservation: reservation.into(),
    }))
}
