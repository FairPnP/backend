use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

// ======================================================================
// DB Entity

#[derive(Debug, FromRow)]
pub struct UserReview {
    pub id: i32,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub message: String,
    pub stars: i32,
    pub last_modified: NaiveDateTime,
    pub created_at: NaiveDateTime,
}
