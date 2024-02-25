#![allow(dead_code)]
use uuid::Uuid;

use crate::services::postgres::DbPool;

use self::entities::UserProfile;

pub mod entities;

pub struct UserProfileDb {}

impl UserProfileDb {
    // ======================================================================
    // Create

    pub async fn insert(
        pool: &DbPool,
        user_id: Uuid,
        name: String,
        avatar_url: Option<String>,
    ) -> Result<UserProfile, sqlx::Error> {
        let user_profile = sqlx::query_as::<_, UserProfile>(
            "INSERT INTO user_profiles (user_id, name, avatar_url) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(user_id)
        .bind(name)
        .bind(avatar_url)
        .fetch_one(pool)
        .await?;

        Ok(user_profile)
    }

    // ======================================================================
    // Read

    pub async fn get(pool: &DbPool, user_id: Uuid) -> Result<UserProfile, sqlx::Error> {
        let user_profile =
            sqlx::query_as::<_, UserProfile>("SELECT * FROM user_profiles WHERE user_id = $1")
                .bind(user_id)
                .fetch_one(pool)
                .await?;

        Ok(user_profile)
    }

    // ======================================================================
    // Update

    pub async fn update(
        pool: &DbPool,
        user_id: Uuid,
        new_name: Option<String>,
        new_avatar_url: Option<String>,
    ) -> Result<UserProfile, sqlx::Error> {
        let user_profile = sqlx::query_as::<_, UserProfile>(
            "UPDATE user_profiles SET name = COALESCE($1, name), avatar_url = COALESCE($2, avatar_url) WHERE user_id = $3 RETURNING *")
            .bind(new_name)
            .bind(new_avatar_url)
            .bind(user_id)
            .fetch_one(pool)
            .await?;

        Ok(user_profile)
    }

    // ======================================================================
    // Delete

    pub async fn delete(pool: &DbPool, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM user_profiles WHERE user_id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
