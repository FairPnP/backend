use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

// ======================================================================
// DB Entity

#[derive(Debug, FromRow)]
pub struct UserNotifToken {
    pub id: i32,
    pub user_id: Uuid,
    pub expo_token: Option<String>,
    pub device_token: String,
    pub device_type: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}
