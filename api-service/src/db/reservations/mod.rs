use chrono::NaiveDateTime;
use uuid::Uuid;

use self::entities::Reservation;

use super::DbPool;

pub mod entities;

pub struct ReservationDb {}

impl ReservationDb {
    // ======================================================================
    // Create

    pub async fn insert(
        pool: &DbPool,
        user_id: Uuid,
        availability_id: i32,
        start_date: NaiveDateTime,
        end_date: NaiveDateTime,
    ) -> Result<Reservation, sqlx::Error> {
        let reservation = sqlx::query_as::<_, Reservation>(
            "INSERT INTO reservations (user_id, availability_id, start_date, end_date) VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(user_id)
        .bind(availability_id)
        .bind(start_date)
        .bind(end_date)
        .fetch_one(pool)
        .await?
        .into();

        Ok(reservation)
    }

    // ======================================================================
    // Read

    pub async fn get(pool: &DbPool, reservation_id: i32) -> Result<Reservation, sqlx::Error> {
        let reservation =
            sqlx::query_as::<_, Reservation>("SELECT * FROM reservations WHERE id = $1")
                .bind(reservation_id)
                .fetch_one(pool)
                .await?;

        Ok(reservation)
    }

    pub async fn list(
        pool: &DbPool,
        offset_id: Option<i32>,
        limit: i32,
        user_id: Option<Uuid>,
        availability_id: Option<i32>,
    ) -> Result<Vec<Reservation>, sqlx::Error> {
        let mut query = String::from("SELECT * FROM reservations");

        let mut conditions = vec![];
        if let Some(ref uid) = user_id {
            conditions.push(format!("user_id = '{}'", uid));
        }
        if let Some(ref bid) = availability_id {
            conditions.push(format!("availability_id = '{}'", bid));
        }
        if let Some(oid) = offset_id {
            conditions.push(format!("id > {}", oid));
        }

        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }

        query.push_str(&format!(" ORDER BY id ASC LIMIT {}", limit));

        let reservations = sqlx::query_as::<_, Reservation>(&query)
            .fetch_all(pool)
            .await?;

        Ok(reservations)
    }

    // ======================================================================
    // Update

    pub async fn update(
        pool: &DbPool,
        user_id: Uuid,
        reservation_id: i32,
        start_date: Option<NaiveDateTime>,
        end_date: Option<NaiveDateTime>,
    ) -> Result<Reservation, sqlx::Error> {
        let reservation = sqlx::query_as::<_, Reservation>(
            "UPDATE reservations SET start_date = COALESCE($1, start_date), end_date = COALESCE($2, end_date) WHERE id = $3 AND user_id = $4 RETURNING *",
        )
        .bind(start_date)
        .bind(end_date)
        .bind(reservation_id)
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(reservation)
    }

    // ======================================================================
    // Delete

    pub async fn delete(
        pool: &DbPool,
        user_id: Uuid,
        reservation_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM reservations WHERE id = $1 AND user_id = $2")
            .bind(reservation_id)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
