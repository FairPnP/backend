use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel_migrations::MigrationHarness;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use std::env;

use crate::error::ServiceError;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);

    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Run migrations
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(_) => (),
        Err(e) => println!("Failed to run migrations: {}", e),
    }

    pool
}

pub fn get_db_connection(
    pool: &DbPool,
) -> Result<PooledConnection<ConnectionManager<PgConnection>>, ServiceError> {
    pool.get().map_err(|_| {
        ServiceError::InternalError("Failed to get db connection from pool".to_string())
    })
}
