#![allow(dead_code)]
use uuid::Uuid;

use crate::db::DbPool;

use self::entities::UserSummary;

pub mod entities;

pub struct UserSummaryDb {}

impl UserSummaryDb {
    // ======================================================================
    // Create

    pub async fn insert(
        pool: &DbPool,
        user_id: Uuid,
        total_reviews: i32,
        average_stars: i32,
    ) -> Result<UserSummary, sqlx::Error> {
        let user_summary = sqlx::query_as::<_, UserSummary>(
            "INSERT INTO user_summaries (user_id, total_reviews, average_stars) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(user_id)
        .bind(total_reviews)
        .bind(average_stars)
        .fetch_one(pool)
        .await?;

        Ok(user_summary)
    }

    // ======================================================================
    // Read

    pub async fn get(pool: &DbPool, user_id: Uuid) -> Result<UserSummary, sqlx::Error> {
        let user_summary =
            sqlx::query_as::<_, UserSummary>("SELECT * FROM user_summaries WHERE user_id = $1")
                .bind(user_id)
                .fetch_one(pool)
                .await?;

        Ok(user_summary)
    }

    pub async fn list(
        pool: &DbPool,
        user_id: Uuid,
        limit: i32,
        offset_id: Option<i32>,
    ) -> Result<Vec<UserSummary>, sqlx::Error> {
        let mut query = String::from("SELECT * FROM user_summaries WHERE user_id = $1");

        // Optionally filter by a minimum id for pagination
        if let Some(offset) = offset_id {
            query += &format!(" AND id > {}", offset);
        }

        query += " ORDER BY created_at DESC";
        query += &format!(" LIMIT {}", limit);

        let user_summaries = sqlx::query_as::<_, UserSummary>(&query)
            .bind(user_id)
            .fetch_all(pool)
            .await?;

        Ok(user_summaries)
    }

    // ======================================================================
    // Update

    pub async fn update(
        pool: &DbPool,
        user_id: Uuid,
        total_reviews: i32,
        average_stars: i32,
    ) -> Result<UserSummary, sqlx::Error> {
        let user_summary = sqlx::query_as::<_, UserSummary>(
            "UPDATE user_summaries SET total_reviews = COALESCE($1, total_reviews), average_stars = COALESCE($2, average_stars) WHERE user_id = $3 RETURNING *")
            .bind(total_reviews)
            .bind(average_stars)
            .bind(user_id)
            .fetch_one(pool)
            .await?;

        Ok(user_summary)
    }

    // ======================================================================
    // Delete

    pub async fn delete(pool: &DbPool, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM user_summaries WHERE user_id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
