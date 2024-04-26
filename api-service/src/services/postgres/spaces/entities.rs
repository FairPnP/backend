use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

// ======================================================================
// DB Entity

#[derive(Debug, FromRow)]
pub struct Space {
    pub id: i32,
    pub user_id: Uuid,
    pub building_id: i32,
    pub name: String,
    pub description: String,
    pub last_modified: NaiveDateTime,
    pub created_at: NaiveDateTime,
}
