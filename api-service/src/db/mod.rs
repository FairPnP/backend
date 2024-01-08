use std::env;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::error::ServiceError;

// pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub type DbPool = Pool<Postgres>;

pub mod availability;
pub mod buildings;
pub mod reservations;
pub mod s3;
pub mod spaces;
pub mod stripe_accounts;
pub mod stripe_customers;

pub async fn establish_connection() -> DbPool {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&database_url)
        .await
        .expect("Failed to create db pool.");

    // Run migrations
    // let mut conn = pool.get().expect("Failed to get DB connection from pool");
    // match conn.run_pending_migrations(MIGRATIONS) {
    //     Ok(_) => (),
    //     Err(e) => println!("Failed to run migrations: {}", e),
    // }

    pool
}

pub async fn do_health_check(pool: &DbPool) -> Result<(), ServiceError> {
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(pool)
        .await?;

    if row.0 != 150 {
        return Err(ServiceError::InternalError(
            "Health check failed".to_string(),
        ));
    }

    Ok(())
}
