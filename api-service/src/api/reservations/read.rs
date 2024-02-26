use crate::{
    error::ServiceError,
    services::postgres::{reservations::ReservationDb, DbPool},
    utils::hashids::decode_id,
};
use actix_web::{get, web, HttpResponse};
use serde::Serialize;

use super::public::PublicReservation;

// ======================================================================
// DTOs

#[derive(Debug, Serialize)]
pub struct ReadReservationResponse {
    reservation: PublicReservation,
}

// ======================================================================
// Route

#[get("/{id}")]
pub async fn read_reservation(
    pool: web::Data<DbPool>,
    reservation_id: web::Path<String>,
) -> Result<HttpResponse, ServiceError> {
    let reservation_id = decode_id(&reservation_id.into_inner())?;
    let reservation = ReservationDb::get(&pool, reservation_id).await?;

    Ok(HttpResponse::Ok().json(ReadReservationResponse {
        reservation: reservation.into(),
    }))
}
