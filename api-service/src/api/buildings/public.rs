use serde::Serialize;
use sqlx::types::BigDecimal;

use crate::db::buildings::entities::Building;

#[derive(Debug, Serialize)]
pub struct PublicBuilding {
    pub id: i32,
    pub name: String,
    pub place_id: String,
    pub latitude: BigDecimal,
    pub longitude: BigDecimal,
}

impl From<Building> for PublicBuilding {
    fn from(building: Building) -> Self {
        PublicBuilding {
            id: building.id,
            name: building.name,
            place_id: building.place_id,
            latitude: building.latitude,
            longitude: building.longitude,
        }
    }
}
