use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

use crate::db::reservation_chat_messages::entities::ReservationChatMessage;

#[derive(Debug, Serialize)]
pub struct PublicChatMessage {
    pub id: i32,
    pub sender_id: Uuid,
    pub message: String,
    pub created_at: NaiveDateTime,
}

impl From<ReservationChatMessage> for PublicChatMessage {
    fn from(message: ReservationChatMessage) -> Self {
        PublicChatMessage {
            id: message.id,
            sender_id: message.sender_id,
            message: message.message,
            created_at: message.created_at,
        }
    }
}
