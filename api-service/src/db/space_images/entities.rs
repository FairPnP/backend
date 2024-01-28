use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// ======================================================================
// DB Entity

#[derive(Debug, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "image_status", rename_all = "lowercase")]
pub enum SpaceImageStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, FromRow)]
pub struct SpaceImage {
    pub id: i32,
    pub space_id: i32,
    pub slot_id: i32,
    pub img_url: String,
    pub status: SpaceImageStatus,
    pub last_modified: NaiveDateTime,
    pub created_at: NaiveDateTime,
}
