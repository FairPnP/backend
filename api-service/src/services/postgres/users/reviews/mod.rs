#![allow(dead_code)]
use crate::services::postgres::DbPool;

use self::entities::UserReview;
use super::summaries::entities::UserSummary;
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub mod entities;

pub struct UserReviewDb {}

impl UserReviewDb {
    // ======================================================================
    // Create

    pub async fn insert(
        pool: &PgPool,
        from_user_id: Uuid,
        to_user_id: Uuid,
        message: String,
        stars: i32,
    ) -> Result<UserReview, Error> {
        let mut tx = pool.begin().await?;

        // Insert the user review
        let review: UserReview = sqlx::query_as::<_, UserReview>(
        "INSERT INTO user_reviews (from_user_id, to_user_id, message, stars) VALUES ($1, $2, $3, $4) RETURNING *",
        )
            .bind(from_user_id)
            .bind(to_user_id)
            .bind(message)
            .bind(stars)
            .fetch_one(&mut *tx)
            .await?;

        // Attempt to fetch existing user summary
        let summary_result =
            sqlx::query_as::<_, UserSummary>("SELECT * FROM user_summaries WHERE user_id = $1")
                .bind(to_user_id)
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

                sqlx::query("UPDATE user_summaries SET total_reviews = $1, average_stars = $2 WHERE user_id = $3")
                    .bind(new_total_reviews)
                    .bind(new_average_stars)
                    .bind(to_user_id)
                    .execute(&mut *tx)
                    .await?;
            }
            None => {
                // average stars is stored as i32, decimal place moved over 2
                let new_average_stars = (review.stars as f64 * 100.0).round() as i32;

                // Insert a new summary if it doesn't exist
                sqlx::query(

                "INSERT INTO user_summaries (user_id, total_reviews, average_stars) VALUES ($1, $2, $3)"
                )
                    .bind(to_user_id)
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
        from_user_id: Uuid,
        to_user_id: Uuid,
    ) -> Result<UserReview, sqlx::Error> {
        let user_review = sqlx::query_as::<_, UserReview>(
            "SELECT * FROM user_reviews WHERE from_user_id = $1 AND to_user_id = $2",
        )
        .bind(from_user_id)
        .bind(to_user_id)
        .fetch_one(pool)
        .await?;

        Ok(user_review)
    }

    pub async fn list(
        pool: &DbPool,
        to_user_id: Uuid,
        limit: i32,
        offset_id: Option<i32>,
    ) -> Result<Vec<UserReview>, sqlx::Error> {
        let mut query = String::from("SELECT * FROM user_reviews WHERE to_user_id = $1");

        // Optionally filter by a minimum id for pagination
        if let Some(offset) = offset_id {
            query += &format!(" AND id > {}", offset);
        }

        query += " ORDER BY created_at DESC";
        query += &format!(" LIMIT {}", limit);

        let user_reviews = sqlx::query_as::<_, UserReview>(&query)
            .bind(to_user_id)
            .fetch_all(pool)
            .await?;

        Ok(user_reviews)
    }

    // ======================================================================
    // Update

    pub async fn update(
        pool: &DbPool,
        from_user_id: Uuid,
        to_user_id: Uuid,
        message: Option<String>,
        stars: Option<i32>,
    ) -> Result<UserReview, sqlx::Error> {
        let mut tx = pool.begin().await?;

        let user_review = match stars {
            Some(stars) => {
                let original_review = sqlx::query_as::<_, UserReview>(
                    "SELECT * FROM user_reviews WHERE from_user_id = $1 AND to_user_id = $2",
                )
                .bind(from_user_id)
                .bind(to_user_id)
                .fetch_one(&mut *tx)
                .await?;

                let new_review = sqlx::query_as::<_, UserReview>(
            "UPDATE user_reviews SET message = COALESCE($1, message), stars = COALESCE($2, stars) WHERE from_user_id = $3 AND to_user_id = $4 RETURNING *")
            .bind(message)
            .bind(stars)
            .bind(from_user_id)
            .bind(to_user_id)
            .fetch_one(&mut *tx)
            .await?;

                // fetch summary
                let summary = sqlx::query_as::<_, UserSummary>(
                    "SELECT * FROM user_summaries WHERE user_id = $1",
                )
                .bind(to_user_id)
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
                sqlx::query("UPDATE user_summaries SET average_stars = $1 WHERE user_id = $2")
                    .bind(new_average_stars)
                    .bind(to_user_id)
                    .execute(&mut *tx)
                    .await?;

                new_review
            }
            None => {
                let new_review = sqlx::query_as::<_, UserReview>(
            "UPDATE user_reviews SET message = COALESCE($1, message) WHERE from_user_id = $2 AND to_user_id = $3 RETURNING *")
            .bind(message)
            .bind(from_user_id)
            .bind(to_user_id)
            .fetch_one(&mut *tx)
            .await?;

                new_review
            }
        };

        tx.commit().await?;

        Ok(user_review)
    }

    // ======================================================================
    // Delete

    pub async fn delete(
        pool: &DbPool,
        from_user_id: Uuid,
        to_user_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        // get original review
        let original_review = sqlx::query_as::<_, UserReview>(
            "SELECT * FROM user_reviews WHERE from_user_id = $1 AND to_user_id = $2",
        )
        .bind(from_user_id)
        .bind(to_user_id)
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query("DELETE FROM user_reviews WHERE from_user_id = $1 AND to_user_id = $2")
            .bind(from_user_id)
            .bind(to_user_id)
            .execute(&mut *tx)
            .await?;

        // get summary
        let summary =
            sqlx::query_as::<_, UserSummary>("SELECT * FROM user_summaries WHERE user_id = $1")
                .bind(to_user_id)
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
            "UPDATE user_summaries SET average_stars = $1, total_reviews = $2 WHERE user_id = $3",
        )
        .bind(new_average_stars)
        .bind(new_total_reviews)
        .bind(to_user_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }
}
