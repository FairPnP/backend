use serde::Serialize;
use sqlx::types::BigDecimal;

use crate::services::postgres::buildings::entities::Building;

#[derive(Debug, Serialize)]
pub struct PublicBuilding {
    pub id: i32,
    pub name: String,
    pub place_id: String,
    pub latitude: BigDecimal,
    pub longitude: BigDecimal,
    pub street_number: String,
    pub street_name: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

impl From<Building> for PublicBuilding {
    fn from(building: Building) -> Self {
        PublicBuilding {
            id: building.id,
            name: building.name,
            place_id: building.place_id,
            latitude: building.latitude,
            longitude: building.longitude,
            street_number: building.street_number,
            street_name: building.street_name,
            city: building.city,
            state: building.state,
            postal_code: building.postal_code,
            country: building.country,
        }
    }
}
