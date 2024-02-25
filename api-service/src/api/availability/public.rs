use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::Serialize;

use crate::services::postgres::availability::entities::Availability;

#[derive(Debug, Serialize)]
pub struct PublicAvailability {
    pub id: i32,
    pub space_id: i32,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub hourly_rate: BigDecimal,
}

impl From<Availability> for PublicAvailability {
    fn from(availability: Availability) -> Self {
        PublicAvailability {
            id: availability.id,
            space_id: availability.space_id,
            start_date: availability.start_date,
            end_date: availability.end_date,
            hourly_rate: availability.hourly_rate,
        }
    }
}
