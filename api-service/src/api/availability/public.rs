use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::Serialize;

use crate::{services::postgres::availability::entities::Availability, utils::hashids::encode_id};

#[derive(Debug, Serialize)]
pub struct PublicAvailability {
    pub id: String,
    pub space_id: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub price: BigDecimal,
}

impl From<Availability> for PublicAvailability {
    fn from(availability: Availability) -> Self {
        PublicAvailability {
            id: encode_id(availability.id),
            space_id: encode_id(availability.space_id),
            start_date: availability.start_date,
            end_date: availability.end_date,
            price: availability.price,
        }
    }
}
