use chrono::NaiveDateTime;
use serde::Serialize;

use crate::{
    services::postgres::reservations::entities::{Reservation, ReservationStatus},
    utils::hashids::encode_id,
};

#[derive(Debug, Serialize)]
pub struct PublicReservation {
    pub id: String,
    pub space_id: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub status: ReservationStatus,
}

impl From<Reservation> for PublicReservation {
    fn from(reservation: Reservation) -> Self {
        PublicReservation {
            id: encode_id(reservation.id),
            space_id: encode_id(reservation.space_id),
            start_date: reservation.start_date,
            end_date: reservation.end_date,
            status: reservation.status,
        }
    }
}
