#![allow(dead_code)]
use uuid::Uuid;

use self::entities::ReservationChatMessage;

use super::DbPool;

pub mod entities;

pub struct ReservationChatMessageDb {}

impl ReservationChatMessageDb {
    // ======================================================================
    // Create

    pub async fn insert(
        pool: &DbPool,
        reservation_id: i32,
        sender_id: Uuid,
        message: String,
    ) -> Result<ReservationChatMessage, sqlx::Error> {
        let chat_message = sqlx::query_as::<_, ReservationChatMessage>(
            "INSERT INTO reservation_chat_messages (reservation_id, sender_id, message) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(reservation_id)
        .bind(sender_id)
        .bind(message)
        .fetch_one(pool)
        .await?;

        Ok(chat_message)
    }

    // ======================================================================
    // Read

    pub async fn get(
        pool: &DbPool,
        message_id: i32,
    ) -> Result<ReservationChatMessage, sqlx::Error> {
        let building = sqlx::query_as::<_, ReservationChatMessage>(
            "SELECT * FROM reservation_chat_messages WHERE id = $1",
        )
        .bind(message_id)
        .fetch_one(pool)
        .await?;

        Ok(building)
    }

    // ======================================================================
    // Update

    pub async fn update(
        pool: &DbPool,
        message_id: i32,
        new_message: Option<String>,
    ) -> Result<ReservationChatMessage, sqlx::Error> {
        let chat_message = sqlx::query_as::<_, ReservationChatMessage>(
            "UPDATE reservation_chat_messages SET message = COALESCE($1, message) WHERE id = $2 RETURNING *",
        )
        .bind(new_message)
        .bind(message_id)
        .fetch_one(pool)
        .await?;

        Ok(chat_message)
    }

    // ======================================================================
    // Delete

    pub async fn delete(pool: &DbPool, message_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM reservation_chat_messages WHERE id = $1")
            .bind(message_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    // ======================================================================
    // List

    pub async fn list(
        pool: &DbPool,
        reservation_id: i32,
        before_id: Option<i32>,
        after_id: Option<i32>,
        limit: i32,
    ) -> Result<Vec<ReservationChatMessage>, sqlx::Error> {
        let mut query = format!(
            "SELECT * FROM reservation_chat_messages WHERE reservation_id = '{}'",
            reservation_id
        );

        if let Some(bid) = before_id {
            query.push_str(&format!(" AND id < {}", bid));
        } else if let Some(aid) = after_id {
            query.push_str(&format!(" AND id > {}", aid));
        }

        query.push_str(" ORDER BY created_at DESC, id DESC");
        query.push_str(&format!(" LIMIT {}", limit));

        let messages = sqlx::query_as::<_, ReservationChatMessage>(&query)
            .fetch_all(pool)
            .await?;

        Ok(messages)
    }
}
