use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::types::BigDecimal;
use sqlx::FromRow;

// ======================================================================
// DB Entity

#[derive(Debug, FromRow)]
pub struct Building {
    pub id: i32,
    pub name: String,
    pub place_id: String,
    pub latitude: BigDecimal,
    pub longitude: BigDecimal,
    pub last_modified: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

// ======================================================================
// Public Entity

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
