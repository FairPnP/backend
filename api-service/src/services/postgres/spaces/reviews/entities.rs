use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

// ======================================================================
// DB Entity

#[derive(Debug, FromRow)]
pub struct SpaceReview {
    pub id: i32,
    pub user_id: Uuid,
    pub space_id: i32,
    pub message: String,
    pub stars: i32,
    pub last_modified: NaiveDateTime,
    pub created_at: NaiveDateTime,
}
