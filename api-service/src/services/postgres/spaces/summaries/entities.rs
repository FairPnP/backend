use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

// ======================================================================
// DB Entity

#[derive(Debug, FromRow)]
pub struct SpaceSummary {
    pub id: i32,
    pub host_user_id: Uuid,
    pub space_id: i32,
    pub total_reviews: i32,
    pub average_stars: i32,
    pub last_modified: NaiveDateTime,
    pub created_at: NaiveDateTime,
}
