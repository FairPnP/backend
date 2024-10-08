use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

// ======================================================================
// DB Entity

#[derive(Debug, FromRow)]
pub struct Availability {
    pub id: i32,
    pub user_id: Uuid,
    pub space_id: i32,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub price: BigDecimal,
    pub created_at: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

// TODO: Clean up these result types, refactor searching in general
#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub building: BuildingResult,
    pub space: SpaceResult,
    pub availability: AvailabilityResult,
}

#[derive(Debug, Serialize)]
pub struct BuildingResult {
    pub id: String,
    pub name: String,
    pub place_id: String,
    pub latitude: BigDecimal,
    pub longitude: BigDecimal,
}

#[derive(Debug, Serialize)]
pub struct SpaceResult {
    pub id: String,
    pub building_id: String,
}

#[derive(Debug, Serialize)]
pub struct AvailabilityResult {
    pub id: String,
    pub space_id: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub price: BigDecimal,
}
