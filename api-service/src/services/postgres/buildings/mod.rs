#![allow(dead_code)]
use bigdecimal::BigDecimal;

use self::entities::Building;

use super::DbPool;

pub mod entities;

pub struct BuildingDb {}

impl BuildingDb {
    // ======================================================================
    // Create

    #[allow(clippy::too_many_arguments)]
    pub async fn insert(
        pool: &DbPool,
        name: String,
        place_id: String,
        latitude: BigDecimal,
        longitude: BigDecimal,
        street_number: String,
        street_name: String,
        city: String,
        state: String,
        postal_code: String,
        country: String,
    ) -> Result<Building, sqlx::Error> {
        let building = sqlx::query_as::<_, Building>(
    "INSERT INTO buildings (name, place_id, latitude, longitude, street_number, street_name, city, state, postal_code, country) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *")
      .bind(&name)
      .bind(&place_id)
      .bind(latitude)
      .bind(longitude)
      .bind(&street_number)
      .bind(&street_name)
      .bind(&city)
      .bind(&state)
      .bind(&postal_code)
      .bind(&country)
      .fetch_one(pool)
      .await?;

        Ok(building)
    }

    // ======================================================================
    // Read

    pub async fn get(pool: &DbPool, building_id: i32) -> Result<Building, sqlx::Error> {
        let building = sqlx::query_as::<_, Building>("SELECT * FROM buildings WHERE id = $1")
            .bind(building_id)
            .fetch_one(pool)
            .await?;

        Ok(building)
    }

    pub async fn list(
        pool: &DbPool,
        offset_id: Option<i32>,
        limit: i32,
        place_id: Option<String>,
        ids: Option<Vec<i32>>,
    ) -> Result<Vec<Building>, sqlx::Error> {
        let mut query = String::from("SELECT * FROM buildings");

        let mut conditions = vec![];
        if let Some(ref pid) = place_id {
            conditions.push(format!("place_id = '{}'", pid));
        }
        if let Some(oid) = offset_id {
            conditions.push(format!("id > {}", oid));
        }
        if let Some(ids) = ids {
            if !ids.is_empty() {
                let ids_list = ids
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                conditions.push(format!("id IN ({})", ids_list));
            }
        }

        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }

        query.push_str(&format!(" ORDER BY id ASC LIMIT {}", limit));

        let buildings = sqlx::query_as::<_, Building>(&query)
            .fetch_all(pool)
            .await?;

        Ok(buildings)
    }

    // ======================================================================
    // Update

    pub async fn update(
        pool: &DbPool,
        building_id: i32,
        name: Option<String>,
        place_id: Option<String>,
        latitude: Option<BigDecimal>,
        longitude: Option<BigDecimal>,
    ) -> Result<Building, sqlx::Error> {
        let building = sqlx::query_as::<_, Building>(
      "UPDATE buildings SET name = COALESCE($1, name), place_id = COALESCE($2, place_id), latitude = COALESCE($3, latitude), longitude = COALESCE($4, longitude) WHERE id = $5 RETURNING *")
      .bind(name)
      .bind(place_id)
      .bind(latitude)
      .bind(longitude)
      .bind(building_id)
      .fetch_one(pool)
      .await?;

        Ok(building)
    }

    // ======================================================================
    // Delete

    pub async fn delete(pool: &DbPool, building_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM buildings WHERE id = $1")
            .bind(building_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
