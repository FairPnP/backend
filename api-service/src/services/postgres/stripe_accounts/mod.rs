#![allow(dead_code)]
use uuid::Uuid;

use self::entities::StripeAccount;

use super::DbPool;

pub mod entities;

pub struct StripeAccountDb {}

impl StripeAccountDb {
    // ======================================================================
    // Create

    pub async fn insert(
        pool: &DbPool,
        user_id: Uuid,
        account_id: &str,
    ) -> Result<StripeAccount, sqlx::Error> {
        let stripe_account = sqlx::query_as::<_, StripeAccount>(
            "INSERT INTO stripe_accounts (user_id, account_id) VALUES ($1, $2) RETURNING *",
        )
        .bind(user_id)
        .bind(account_id)
        .fetch_one(pool)
        .await?;

        Ok(stripe_account)
    }

    // ======================================================================
    // Read

    pub async fn get(pool: &DbPool, user_id: Uuid) -> Result<StripeAccount, sqlx::Error> {
        let stripe_account =
            sqlx::query_as::<_, StripeAccount>("SELECT * FROM stripe_accounts WHERE user_id = $1")
                .bind(user_id)
                .fetch_one(pool)
                .await?;

        Ok(stripe_account)
    }

    // ======================================================================
    // Update

    pub async fn update(
        pool: &DbPool,
        user_id: Uuid,
        account_id: Option<String>,
    ) -> Result<StripeAccount, sqlx::Error> {
        let stripe_account = sqlx::query_as::<_, StripeAccount>(
      "UPDATE stripe_accounts SET account_id = COALESCE($1, account_id) WHERE user_id = $2 RETURNING *")
      .bind(account_id)
      .bind(user_id)
      .fetch_one(pool)
      .await?;

        Ok(stripe_account)
    }

    // ======================================================================
    // Delete

    pub async fn delete(pool: &DbPool, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM stripe_accounts WHERE user_id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
