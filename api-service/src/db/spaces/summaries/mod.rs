#![allow(dead_code)]
use self::entities::SpaceSummary;
use super::DbPool;
use uuid::Uuid;

pub mod entities;

pub struct SpaceSummaryDb {}

impl SpaceSummaryDb {
    // ======================================================================
    // Create

    pub async fn insert(
        pool: &DbPool,
        user_id: Uuid,
        space_id: i32,
        message: String,
        stars: i32,
    ) -> Result<SpaceSummary, sqlx::Error> {
        let space_summary = sqlx::query_as::<_, SpaceSummary>(
            "INSERT INTO space_summaries (user_id, space_id, message, stars) VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(user_id)
        .bind(space_id)
        .bind(message)
        .bind(stars)
        .fetch_one(pool)
        .await?;

        Ok(space_summary)
    }

    // ======================================================================
    // Read

    pub async fn get(pool: &DbPool, space_id: i32) -> Result<SpaceSummary, sqlx::Error> {
        let space_summary =
            sqlx::query_as::<_, SpaceSummary>("SELECT * FROM space_summaries WHERE space_id = $1")
                .bind(space_id)
                .fetch_one(pool)
                .await?;

        Ok(space_summary)
    }

    pub async fn list(
        pool: &DbPool,
        space_id: i32,
        limit: i32,
        offset_id: Option<i32>,
    ) -> Result<Vec<SpaceSummary>, sqlx::Error> {
        let mut query = String::from("SELECT * FROM space_summaries WHERE space_id = $1");

        // Optionally filter by a minimum id for pagination
        if let Some(offset) = offset_id {
            query += &format!(" AND id > {}", offset);
        }

        query += " ORDER BY created_at DESC";
        query += &format!(" LIMIT {}", limit);

        let space_summaries = sqlx::query_as::<_, SpaceSummary>(&query)
            .bind(space_id)
            .fetch_all(pool)
            .await?;

        Ok(space_summaries)
    }

    // ======================================================================
    // Update

    pub async fn update(
        pool: &DbPool,
        space_id: i32,
        message: Option<String>,
        stars: Option<i32>,
    ) -> Result<SpaceSummary, sqlx::Error> {
        let space_summary = sqlx::query_as::<_, SpaceSummary>(
            "UPDATE space_summaries SET message = COALESCE($1, message), stars = COALESCE($2, stars) WHERE space_id = $3 RETURNING *")
            .bind(message)
            .bind(stars)
            .bind(space_id)
            .fetch_one(pool)
            .await?;

        Ok(space_summary)
    }

    // ======================================================================
    // Delete

    pub async fn delete(pool: &DbPool, space_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM space_summaries WHERE space_id = $1")
            .bind(space_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
