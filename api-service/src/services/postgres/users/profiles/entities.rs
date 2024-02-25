use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

// ======================================================================
// DB Entity

#[derive(Debug, FromRow)]
pub struct UserProfile {
    pub id: i32,
    pub user_id: Uuid,
    pub name: String,
    pub avatar_url: Option<String>,
    pub last_modified: NaiveDateTime,
    pub created_at: NaiveDateTime,
}
