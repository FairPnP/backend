#![allow(dead_code)]
use std::collections::HashMap;

use self::entities::{SpaceImage, SpaceImageStatus};

use super::DbPool;

pub mod entities;

pub struct SpaceImageDb {}

impl SpaceImageDb {
    // ======================================================================
    // Create

    pub async fn insert(
        pool: &DbPool,
        space_id: i32,
        slot_id: i32,
        img_url: String,
        status: SpaceImageStatus,
    ) -> Result<SpaceImage, sqlx::Error> {
        let space_image = sqlx::query_as::<_, SpaceImage>(
            "INSERT INTO space_images (space_id, slot_id, img_url, status) VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(space_id)
        .bind(slot_id)
        .bind(img_url)
        .bind(status)
        .fetch_one(pool)
        .await?;

        Ok(space_image)
    }

    // ======================================================================
    // Read

    pub async fn get(pool: &DbPool, space_image_id: i32) -> Result<SpaceImage, sqlx::Error> {
        let space_image =
            sqlx::query_as::<_, SpaceImage>("SELECT * FROM space_images WHERE id = $1")
                .bind(space_image_id)
                .fetch_one(pool)
                .await?;

        Ok(space_image)
    }

    pub async fn list(
        pool: &DbPool,
        ids: &Vec<i32>,
        status: SpaceImageStatus,
    ) -> Result<Vec<SpaceImage>, sqlx::Error> {
        let space_images = sqlx::query_as::<_, SpaceImage>(
            "SELECT * FROM space_images WHERE id = ANY($1) AND status = $2 ORDER BY slot_id ASC",
        )
        .bind(ids)
        .bind(status)
        .fetch_all(pool)
        .await?;

        Ok(space_images)
    }

    pub async fn list_for_space(
        pool: &DbPool,
        space_id: i32,
    ) -> Result<Vec<SpaceImage>, sqlx::Error> {
        let space_images = sqlx::query_as::<_, SpaceImage>(
            "SELECT * FROM space_images WHERE space_id = $1 AND status = 'approved' ORDER BY slot_id ASC",
        )
        .bind(space_id)
        .fetch_all(pool)
        .await?;

        Ok(space_images)
    }

    pub async fn list_for_spaces(
        pool: &DbPool,
        space_ids: &Vec<i32>,
    ) -> Result<HashMap<i32, Vec<SpaceImage>>, sqlx::Error> {
        let space_images = sqlx::query_as::<_, SpaceImage>(
            "SELECT * FROM space_images WHERE space_id = ANY($1) AND status = 'approved' ORDER BY slot_id ASC",
        )
        .bind(space_ids)
        .fetch_all(pool)
        .await?;

        let mut space_images_map = HashMap::new();
        for space_image in space_images {
            let space_id = space_image.space_id;
            let space_images = space_images_map.entry(space_id).or_insert(vec![]);
            space_images.push(space_image);
        }

        Ok(space_images_map)
    }

    // ======================================================================
    // Update

    pub async fn update(
        pool: &DbPool,
        space_image_id: i32,
        new_img_url: Option<String>,
        new_status: Option<SpaceImageStatus>,
    ) -> Result<SpaceImage, sqlx::Error> {
        let space_image = sqlx::query_as::<_, SpaceImage>(
            "UPDATE space_images SET img_url = COALESCE($1, img_url), status = COALESCE($2, status) WHERE id = $3 RETURNING *")
            .bind(new_img_url)
            .bind(new_status)
            .bind(space_image_id)
            .fetch_one(pool)
            .await?;

        Ok(space_image)
    }

    pub async fn approve_many(
        pool: &DbPool,
        space_image_ids: &Vec<i32>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE space_images SET status = $1 WHERE id = ANY($2)")
            .bind(SpaceImageStatus::Approved as SpaceImageStatus)
            .bind(space_image_ids)
            .execute(pool)
            .await?;

        Ok(())
    }

    // ======================================================================
    // Delete

    pub async fn delete(pool: &DbPool, space_image_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM space_images WHERE id = $1")
            .bind(space_image_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
