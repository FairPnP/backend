use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use uuid::Uuid;

use self::entities::Availability;

use super::DbPool;

pub mod entities;

pub struct AvailabilityDb {}

impl AvailabilityDb {
    // ======================================================================
    // Create

    pub async fn insert(
        pool: &DbPool,
        user_id: Uuid,
        space_id: i32,
        start_date: NaiveDateTime,
        end_date: NaiveDateTime,
        hourly_rate: BigDecimal,
    ) -> Result<Availability, sqlx::Error> {
        let availability = sqlx::query_as::<_, Availability>(
            "INSERT INTO availability (user_id, space_id, start_date, end_date, hourly_rate) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        )
        .bind(user_id)
        .bind(space_id)
        .bind(start_date)
        .bind(end_date)
        .bind(hourly_rate)
        .fetch_one(pool)
        .await?;

        Ok(availability)
    }

    // ======================================================================
    // Read

    pub async fn get(pool: &DbPool, availability_id: i32) -> Result<Availability, sqlx::Error> {
        let availability =
            sqlx::query_as::<_, Availability>("SELECT * FROM availability WHERE id = $1")
                .bind(availability_id)
                .fetch_one(pool)
                .await?;

        Ok(availability)
    }

    pub async fn list(
        pool: &DbPool,
        offset_id: Option<i32>,
        limit: i64,
        user_id: Option<Uuid>,
        space_id: Option<i32>,
    ) -> Result<Vec<Availability>, sqlx::Error> {
        let mut query = String::from("SELECT * FROM availability");

        let mut conditions = vec![];
        if let Some(ref uid) = user_id {
            conditions.push(format!("user_id = '{}'", uid));
        }
        if let Some(oid) = offset_id {
            if let Some(ref sid) = space_id {
                conditions.push(format!("space_id = '{}'", sid));
            }
            conditions.push(format!("id > {}", oid));
        }

        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }

        query.push_str(&format!(" ORDER BY id ASC LIMIT {}", limit));

        let availability = sqlx::query_as::<_, Availability>(&query)
            .fetch_all(pool)
            .await?;

        Ok(availability)
    }

    // ======================================================================
    // Update

    pub async fn update(
        pool: &DbPool,
        user_id: Uuid,
        availability_id: i32,
        start_date: Option<NaiveDateTime>,
        end_date: Option<NaiveDateTime>,
        hourly_rate: Option<BigDecimal>,
    ) -> Result<Availability, sqlx::Error> {
        let availability = sqlx::query_as::<_, Availability>(
            "UPDATE availability SET start_date = COALESCE($1, start_date), end_date = COALESCE($2, end_date), hourly_rate = COALESCE($3, hourly_rate) WHERE id = $4 AND user_id = $5 RETURNING *",
        )
        .bind(start_date)
        .bind(end_date)
        .bind(hourly_rate)
        .bind(availability_id)
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(availability)
    }

    // ======================================================================
    // Delete

    pub async fn delete(
        pool: &DbPool,
        user_id: Uuid,
        availability_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM availability WHERE id = $1 AND user_id = $2")
            .bind(availability_id)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
