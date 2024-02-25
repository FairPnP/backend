use self::entities::SpaceReview;
use super::{summaries::entities::SpaceSummary, DbPool};
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub mod entities;

pub struct SpaceReviewDb {}

impl SpaceReviewDb {
    // ======================================================================
    // Create

    pub async fn insert(
        pool: &PgPool,
        user_id: Uuid,
        space_id: i32,
        message: String,
        stars: i32,
    ) -> Result<SpaceReview, Error> {
        let mut tx = pool.begin().await?;

        // Insert the space review
        let review: SpaceReview = sqlx::query_as::<_, SpaceReview>(
        "INSERT INTO space_reviews (user_id, space_id, message, stars) VALUES ($1, $2, $3, $4) RETURNING *",
        )
            .bind(user_id)
            .bind(space_id)
            .bind(message)
            .bind(stars)
            .fetch_one(&mut *tx)
            .await?;

        // Attempt to fetch existing space summary
        let summary_result =
            sqlx::query_as::<_, SpaceSummary>("SELECT * FROM space_summaries WHERE space_id = $1")
                .bind(space_id)
                .fetch_optional(&mut *tx)
                .await?;

        match summary_result {
            Some(summary) => {
                // Update the existing summary
                let new_total_reviews = summary.total_reviews + 1;
                let total_stars =
                    (summary.average_stars as f64 / 100.0) * summary.total_reviews as f64;

                // average stars is stored as i32, decimal place moved over 2
                let new_average_stars = ((total_stars + review.stars as f64)
                    / new_total_reviews as f64
                    * 100.0)
                    .round() as i32;

                sqlx::query("UPDATE space_summaries SET total_reviews = $1, average_stars = $2 WHERE space_id = $3")
                    .bind(new_total_reviews)
                    .bind(new_average_stars)
                    .bind(space_id)
                    .execute(&mut *tx)
                    .await?;
            }
            None => {
                // average stars is stored as i32, decimal place moved over 2
                let new_average_stars = (review.stars as f64 * 100.0).round() as i32;

                // Insert a new summary if it doesn't exist
                sqlx::query(

                "INSERT INTO space_summaries (host_user_id, space_id, total_reviews, average_stars) VALUES ($1, $2, $3, $4)"
                )
                    .bind(user_id)
                    .bind(space_id)
                    .bind(1)
                    .bind(new_average_stars)
                    .execute(&mut *tx)
                    .await?;
            }
        }

        // Commit the transaction
        tx.commit().await?;

        Ok(review)
    }

    // ======================================================================
    // Read

    pub async fn get(
        pool: &DbPool,
        user_id: Uuid,
        space_id: i32,
    ) -> Result<SpaceReview, sqlx::Error> {
        let space_review = sqlx::query_as::<_, SpaceReview>(
            "SELECT * FROM space_reviews WHERE user_id = $1 AND space_id = $2",
        )
        .bind(user_id)
        .bind(space_id)
        .fetch_one(pool)
        .await?;

        Ok(space_review)
    }

    pub async fn list(
        pool: &DbPool,
        space_id: i32,
        limit: i32,
        offset_id: Option<i32>,
    ) -> Result<Vec<SpaceReview>, sqlx::Error> {
        let mut query = String::from("SELECT * FROM space_reviews WHERE space_id = $1");

        // Optionally filter by a minimum id for pagination
        if let Some(offset) = offset_id {
            query += &format!(" AND id > {}", offset);
        }

        query += " ORDER BY created_at DESC";
        query += &format!(" LIMIT {}", limit);

        let space_reviews = sqlx::query_as::<_, SpaceReview>(&query)
            .bind(space_id)
            .fetch_all(pool)
            .await?;

        Ok(space_reviews)
    }

    // ======================================================================
    // Update

    pub async fn update(
        pool: &DbPool,
        user_id: Uuid,
        space_id: i32,
        message: Option<String>,
        stars: Option<i32>,
    ) -> Result<SpaceReview, sqlx::Error> {
        let mut tx = pool.begin().await?;

        let space_review = match stars {
            Some(stars) => {
                let original_review = sqlx::query_as::<_, SpaceReview>(
                    "SELECT * FROM space_reviews WHERE user_id = $1 AND space_id = $2",
                )
                .bind(user_id)
                .bind(space_id)
                .fetch_one(&mut *tx)
                .await?;

                let new_review = sqlx::query_as::<_, SpaceReview>(
            "UPDATE space_reviews SET message = COALESCE($1, message), stars = COALESCE($2, stars) WHERE user_id = $3 AND space_id = $4 RETURNING *")
            .bind(message)
            .bind(stars)
            .bind(user_id)
            .bind(space_id)
            .fetch_one(&mut *tx)
            .await?;

                // fetch summary
                let summary = sqlx::query_as::<_, SpaceSummary>(
                    "SELECT * FROM space_summaries WHERE space_id = $1",
                )
                .bind(space_id)
                .fetch_one(&mut *tx)
                .await?;

                // calculate new average stars
                let average_stars =
                    (summary.average_stars as f64 / 100.0) * summary.total_reviews as f64;
                let new_average_stars = ((average_stars - original_review.stars as f64
                    + new_review.stars as f64)
                    / summary.total_reviews as f64
                    * 100.0)
                    .round() as i32;

                // update summary
                sqlx::query("UPDATE space_summaries SET average_stars = $1 WHERE space_id = $2")
                    .bind(new_average_stars)
                    .bind(space_id)
                    .execute(&mut *tx)
                    .await?;

                new_review
            }
            None => {
                let new_review = sqlx::query_as::<_, SpaceReview>(
            "UPDATE space_reviews SET message = COALESCE($1, message) WHERE user_id = $2 AND space_id = $3 RETURNING *")
            .bind(message)
            .bind(user_id)
            .bind(space_id)
            .fetch_one(&mut *tx)
            .await?;

                new_review
            }
        };

        tx.commit().await?;

        Ok(space_review)
    }

    // ======================================================================
    // Delete

    pub async fn delete(pool: &DbPool, user_id: Uuid, space_id: i32) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        // get original review
        let original_review = sqlx::query_as::<_, SpaceReview>(
            "SELECT * FROM space_reviews WHERE user_id = $1 AND space_id = $2",
        )
        .bind(user_id)
        .bind(space_id)
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query("DELETE FROM space_reviews WHERE user_id = $1 AND space_id = $2")
            .bind(user_id)
            .bind(space_id)
            .execute(&mut *tx)
            .await?;

        // get summary
        let summary =
            sqlx::query_as::<_, SpaceSummary>("SELECT * FROM space_summaries WHERE space_id = $1")
                .bind(space_id)
                .fetch_one(&mut *tx)
                .await?;

        // calculate new average stars
        let average_stars = (summary.average_stars as f64 / 100.0) * summary.total_reviews as f64;
        let new_average_stars = ((average_stars - original_review.stars as f64)
            / (summary.total_reviews - 1) as f64
            * 100.0)
            .round() as i32;
        let new_total_reviews = summary.total_reviews - 1;

        // update summary
        sqlx::query(
            "UPDATE space_summaries SET average_stars = $1, total_reviews = $2 WHERE space_id = $3",
        )
        .bind(new_average_stars)
        .bind(new_total_reviews)
        .bind(space_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }
}
