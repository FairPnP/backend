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
    pub description: Option<String>,
    pub max_vehicle_size: String,
    pub coverage: String,
    pub height_clearance_cm: Option<i32>,
    pub access_restrictions: Option<String>,
    pub parking_instructions: Option<String>,
    pub last_modified: NaiveDateTime,
    pub created_at: NaiveDateTime,
}
