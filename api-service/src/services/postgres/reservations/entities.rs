use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

// ======================================================================
// DB Entity

#[derive(Debug, FromRow)]
pub struct Reservation {
    pub id: i32,
    pub user_id: Uuid,
    pub space_id: i32,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub last_modified: NaiveDateTime,
    pub created_at: NaiveDateTime,
}
