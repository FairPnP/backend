use uuid::Uuid;

use self::entities::Space;

use super::DbPool;

pub mod entities;

pub struct SpaceDb {}

impl SpaceDb {
    // ======================================================================
    // Create

    pub async fn insert(
        pool: &DbPool,
        user_id: Uuid,
        building_id: i32,
        name: String,
    ) -> Result<Space, sqlx::Error> {
        let space = sqlx::query_as::<_, Space>(
            "INSERT INTO spaces (user_id, building_id, name) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(user_id)
        .bind(building_id)
        .bind(&name)
        .fetch_one(pool)
        .await?
        .into();

        Ok(space)
    }

    // ======================================================================
    // Read

    pub async fn get(pool: &DbPool, space_id: i32) -> Result<Space, sqlx::Error> {
        let space = sqlx::query_as::<_, Space>("SELECT * FROM spaces WHERE id = $1")
            .bind(space_id)
            .fetch_one(pool)
            .await?;

        Ok(space)
    }

    pub async fn list(
        pool: &DbPool,
        offset_id: Option<i32>,
        limit: i64,
        user_id: Option<Uuid>,
        building_id: Option<String>,
    ) -> Result<Vec<Space>, sqlx::Error> {
        let mut query = String::from("SELECT * FROM spaces");

        let mut conditions = vec![];
        if let Some(ref uid) = user_id {
            conditions.push(format!("user_id = '{}'", uid));
        }
        if let Some(ref bid) = building_id {
            conditions.push(format!("building_id = '{}'", bid));
        }
        if let Some(oid) = offset_id {
            conditions.push(format!("id > {}", oid));
        }

        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }

        query.push_str(&format!(" ORDER BY id ASC LIMIT {}", limit));

        let spaces = sqlx::query_as::<_, Space>(&query).fetch_all(pool).await?;

        Ok(spaces)
    }

    // ======================================================================
    // Update

    pub async fn update(
        pool: &DbPool,
        user_id: Uuid,
        space_id: i32,
        name: Option<String>,
    ) -> Result<Space, sqlx::Error> {
        let space = sqlx::query_as::<_, Space>(
            "UPDATE spaces SET name = COALESCE($1, name) WHERE id = $2 AND user_id = $3 RETURNING *",
        )
        .bind(name)
        .bind(space_id)
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(space)
    }

    // ======================================================================
    // Delete

    pub async fn delete(pool: &DbPool, user_id: Uuid, space_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM spaces WHERE id = $1 AND user_id = $2")
            .bind(space_id)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
