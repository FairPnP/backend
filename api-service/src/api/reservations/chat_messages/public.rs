use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

use crate::db::reservation_chat_messages::entities::{ConversationSummary, ReservationChatMessage};

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

#[derive(Debug, Serialize)]
pub struct PublicConversationSummary {
    pub reservation_id: i32,
    pub user_id: Uuid,
    pub message_id: Option<i32>,
    pub message: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

impl From<ConversationSummary> for PublicConversationSummary {
    fn from(summary: ConversationSummary) -> Self {
        PublicConversationSummary {
            reservation_id: summary.reservation_id,
            user_id: summary.user_id,
            message_id: summary.message_id,
            message: summary.message,
            created_at: summary.created_at,
        }
    }
}
