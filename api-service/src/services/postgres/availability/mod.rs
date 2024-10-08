use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use sqlx::Row;
use uuid::Uuid;

use crate::utils::hashids::encode_id;

use self::entities::{Availability, AvailabilityResult, BuildingResult, SearchResult, SpaceResult};

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
        price: BigDecimal,
    ) -> Result<Availability, sqlx::Error> {
        let availability = sqlx::query_as::<_, Availability>(
            "INSERT INTO availability (user_id, space_id, start_date, end_date, price) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        )
        .bind(user_id)
        .bind(space_id)
        .bind(start_date)
        .bind(end_date)
        .bind(price)
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
        limit: i32,
        user_id: Option<Uuid>,
        space_id: Option<i32>,
    ) -> Result<Vec<Availability>, sqlx::Error> {
        let mut query = String::from("SELECT * FROM availability");

        let mut conditions = vec![];
        if let Some(ref uid) = user_id {
            conditions.push(format!("user_id = '{}'", uid));
        }
        if let Some(ref sid) = space_id {
            conditions.push(format!("space_id = '{}'", sid));
        }
        if let Some(oid) = offset_id {
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

    pub async fn find_overlapping_availabilities(
        pool: &DbPool,
        space_id: i32,
        start_date: NaiveDateTime,
        end_date: NaiveDateTime,
    ) -> Result<Vec<Availability>, sqlx::Error> {
        let query = "
            SELECT * FROM availability
            WHERE space_id = $1
            AND NOT (end_date <= $2 OR start_date >= $3)
            ORDER BY start_date ASC";

        let overlapping_availabilities = sqlx::query_as::<_, Availability>(query)
            .bind(space_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_all(pool)
            .await?;

        Ok(overlapping_availabilities)
    }

    // ======================================================================
    // Update

    pub async fn update(
        pool: &DbPool,
        user_id: Uuid,
        availability_id: i32,
        start_date: Option<NaiveDateTime>,
        end_date: Option<NaiveDateTime>,
        price: Option<BigDecimal>,
    ) -> Result<Availability, sqlx::Error> {
        let availability = sqlx::query_as::<_, Availability>(
            "UPDATE availability SET start_date = COALESCE($1, start_date), end_date = COALESCE($2, end_date), price = COALESCE($3, price) WHERE id = $4 AND user_id = $5 RETURNING *",
        )
        .bind(start_date)
        .bind(end_date)
        .bind(price)
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

    // ======================================================================
    // Search

    pub async fn search(
        pool: &DbPool,
        start_date: NaiveDateTime,
        end_date: NaiveDateTime,
        latitude: BigDecimal,
        longitude: BigDecimal,
        lat_delta: BigDecimal,
        long_delta: BigDecimal,
    ) -> Result<Vec<SearchResult>, sqlx::Error> {
        let query = "
            SELECT 
                a.id as a_id, a.space_id as a_space_id, a.start_date as a_start_date, a.end_date as a_end_date, a.price as a_price,
                b.id as b_id, b.name as b_name, b.place_id as b_place_id, b.latitude as b_latitude, b.longitude as b_longitude,
                s.id as s_id, s.building_id as s_building_id
            FROM buildings b
            JOIN spaces s ON b.id = s.building_id
            JOIN availability a ON s.id = a.space_id
            LEFT JOIN reservations r ON a.id = r.availability_id 
                AND r.status IN ('confirmed', 'pending')
            WHERE b.latitude BETWEEN $3 - $5 AND $3 + $5
            AND b.longitude BETWEEN $4 - $6 AND $4 + $6
            AND r.id IS NULL";

        let rows = sqlx::query(query)
            .bind(start_date)
            .bind(end_date)
            .bind(latitude)
            .bind(longitude)
            .bind(lat_delta)
            .bind(long_delta)
            .fetch_all(pool)
            .await?;

        let mut results = Vec::new();

        // TODO: refactor search results, I don't like encoding at db level
        for row in rows {
            let availability = AvailabilityResult {
                id: encode_id(row.try_get("a_id")?),
                space_id: encode_id(row.try_get("a_space_id")?),
                start_date: row.try_get("a_start_date")?,
                end_date: row.try_get("a_end_date")?,
                price: row.try_get("a_price")?,
            };

            let building = BuildingResult {
                id: encode_id(row.try_get("b_id")?),
                name: row.try_get("b_name")?,
                place_id: row.try_get("b_place_id")?,
                latitude: row.try_get("b_latitude")?,
                longitude: row.try_get("b_longitude")?,
            };

            let space = SpaceResult {
                id: encode_id(row.try_get("s_id")?),
                building_id: encode_id(row.try_get("s_building_id")?),
            };

            results.push(SearchResult {
                availability,
                building,
                space,
            });
        }

        Ok(results)
    }
}
