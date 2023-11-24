use bigdecimal::BigDecimal;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Queryable, Insertable, Identifiable)]
#[diesel(table_name = crate::schema::buildings)]
pub struct Building {
    pub id: i32,
    pub name: String,
    pub latitude: BigDecimal,
    pub longitude: BigDecimal,
    pub place_id: String,
    pub last_modified: std::time::SystemTime,
    pub created_at: std::time::SystemTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::buildings)]
pub struct NewBuilding {
    pub name: String,
    pub latitude: BigDecimal,
    pub longitude: BigDecimal,
    pub place_id: String,
}
