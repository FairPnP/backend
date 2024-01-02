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
    pub hourly_rate: BigDecimal,
    pub created_at: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

#[derive(Serialize)]
pub struct SearchResult {
    pub building: BuildingResult,
    pub space: SpaceResult,
    pub availability: AvailabilityResult,
}

#[derive(Serialize)]
pub struct BuildingResult {
    pub id: i32,
    pub name: String,
    pub place_id: String,
    pub latitude: BigDecimal,
    pub longitude: BigDecimal,
}

#[derive(Debug, Serialize)]
pub struct SpaceResult {
    pub id: i32,
    pub building_id: i32,
}

#[derive(Serialize)]
pub struct AvailabilityResult {
    pub id: i32,
    pub space_id: i32,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub hourly_rate: BigDecimal,
}
