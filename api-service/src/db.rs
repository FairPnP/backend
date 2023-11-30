use std::env;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

// pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub type DbPool = Pool<Postgres>;

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
