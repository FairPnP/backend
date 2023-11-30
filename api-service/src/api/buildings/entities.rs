use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

// ======================================================================
// DB Entity

#[derive(Debug, Queryable, Insertable, Identifiable)]
#[diesel(table_name = crate::schema::buildings)]
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

// ======================================================================
// DB Operations

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::buildings)]
pub struct NewBuilding {
    pub name: String,
    pub place_id: String,
    pub latitude: BigDecimal,
    pub longitude: BigDecimal,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::buildings)]
pub struct UpdateBuilding {
    pub name: Option<String>,
    pub place_id: Option<String>,
    pub latitude: Option<BigDecimal>,
    pub longitude: Option<BigDecimal>,
}
