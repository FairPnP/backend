use chrono::NaiveDateTime;
use serde::Serialize;

use crate::services::postgres::reservations::entities::Reservation;

#[derive(Debug, Serialize)]
pub struct PublicReservation {
    pub id: i32,
    pub space_id: i32,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
}

impl From<Reservation> for PublicReservation {
    fn from(reservation: Reservation) -> Self {
        PublicReservation {
            id: reservation.id,
            space_id: reservation.space_id,
            start_date: reservation.start_date,
            end_date: reservation.end_date,
        }
    }
}
