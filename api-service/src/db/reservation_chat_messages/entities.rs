use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

// ======================================================================
// DB Entity

#[derive(Debug, FromRow)]
pub struct ReservationChatMessage {
    pub id: i32,
    pub reservation_id: i32,
    pub sender_id: Uuid,
    pub message: String,
    pub created_at: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

#[derive(sqlx::FromRow)]
pub struct ConversationSummary {
    pub reservation_id: i32,
    pub user_id: Uuid,
    pub message_id: Option<i32>,
    pub message: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}
