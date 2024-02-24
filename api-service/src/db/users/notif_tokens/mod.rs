#![allow(dead_code)]
use uuid::Uuid;

use crate::db::DbPool;

pub mod entities;

use self::entities::UserNotifToken;

pub struct UserNotifTokenDb {}

impl UserNotifTokenDb {
    // ======================================================================
    // List

    pub async fn list(pool: &DbPool, user_id: Uuid) -> Result<Vec<UserNotifToken>, sqlx::Error> {
        let user_notif_tokens = sqlx::query_as::<_, UserNotifToken>(
            "SELECT * FROM user_notif_tokens WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(user_notif_tokens)
    }

    // ======================================================================
    // Upsert

    pub async fn upsert(
        pool: &DbPool,
        user_id: Uuid,
        expo_token: Option<String>,
        device_token: String,
        device_type: String,
    ) -> Result<UserNotifToken, sqlx::Error> {
        let user_notif_token = sqlx::query_as::<_, UserNotifToken>(
            "INSERT INTO user_notif_tokens (user_id, expo_token, device_token, device_type) VALUES ($1, $2, $3, $4) ON CONFLICT (user_id, expo_token, device_token) DO UPDATE SET last_modified = NOW(), status = 'active' RETURNING *")
            .bind(user_id)
            .bind(expo_token)
            .bind(device_token)
            .bind(device_type)
            .fetch_one(pool)
            .await?;

        Ok(user_notif_token)
    }

    // ======================================================================
    // Expire

    pub async fn expire(pool: &DbPool, id: i32) -> Result<UserNotifToken, sqlx::Error> {
        let user_notif_token = sqlx::query_as::<_, UserNotifToken>(
            "UPDATE user_notif_tokens SET status = $1 WHERE id = $2 RETURNING *",
        )
        .bind("expired")
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(user_notif_token)
    }
}
