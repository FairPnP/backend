use crate::{
    db::{reservations::ReservationDb, DbPool},
    error::ServiceError,
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
    reservation_id: web::Path<i32>,
) -> Result<HttpResponse, ServiceError> {
    let reservation = ReservationDb::get(&pool, reservation_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ReadReservationResponse {
        reservation: reservation.into(),
    }))
}
