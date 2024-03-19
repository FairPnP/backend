use crate::{
    api::validation::validate_req_data,
    auth::user::get_user_id,
    error::ServiceError,
    services::postgres::{reservations::ReservationDb, DbPool},
    utils::hashids::decode_id,
};
use actix_web::{put, web, HttpResponse};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicReservation;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateReservationRequest {
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize)]
pub struct UpdateReservationResponse {
    reservation: PublicReservation,
}

// ======================================================================
// Route

#[put("/{id}")]
pub async fn update_reservation(
    pool: web::Data<DbPool>,
    reservation_id: web::Path<String>,
    req: actix_web::HttpRequest,
    data: web::Json<UpdateReservationRequest>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let data = validate_req_data(data.into_inner())?;
    let reservation_id = decode_id(&reservation_id.into_inner())?;

    let updated_reservation = ReservationDb::update(
        &pool,
        user_id,
        reservation_id,
        data.start_date,
        data.end_date,
        None,
    )
    .await?;
    Ok(HttpResponse::Ok().json(UpdateReservationResponse {
        reservation: updated_reservation.into(),
    }))
}
