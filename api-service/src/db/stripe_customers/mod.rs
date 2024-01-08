#![allow(dead_code)]
use uuid::Uuid;

use self::entities::StripeCustomer;

use super::DbPool;

pub mod entities;

pub struct StripeCustomerDb {}

impl StripeCustomerDb {
    // ======================================================================
    // Create

    pub async fn insert(
        pool: &DbPool,
        user_id: Uuid,
        customer_id: &str,
    ) -> Result<StripeCustomer, sqlx::Error> {
        let stripe_customer = sqlx::query_as::<_, StripeCustomer>(
            "INSERT INTO stripe_customers (user_id, customer_id) VALUES ($1, $2) RETURNING *",
        )
        .bind(&user_id)
        .bind(customer_id)
        .fetch_one(pool)
        .await?
        .into();

        Ok(stripe_customer)
    }

    // ======================================================================
    // Read

    pub async fn get(pool: &DbPool, user_id: Uuid) -> Result<StripeCustomer, sqlx::Error> {
        let stripe_customer = sqlx::query_as::<_, StripeCustomer>(
            "SELECT * FROM stripe_customers WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(stripe_customer)
    }

    // ======================================================================
    // Update

    pub async fn update(
        pool: &DbPool,
        user_id: Uuid,
        customer_id: Option<String>,
    ) -> Result<StripeCustomer, sqlx::Error> {
        let stripe_customer = sqlx::query_as::<_, StripeCustomer>(
      "UPDATE stripe_customers SET customer_id = COALESCE($1, customer_id) WHERE user_id = $2 RETURNING *")
      .bind(customer_id)
      .bind(user_id)
      .fetch_one(pool)
      .await?;

        Ok(stripe_customer)
    }

    // ======================================================================
    // Delete

    pub async fn delete(pool: &DbPool, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM stripe_customers WHERE user_id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
