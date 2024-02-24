use std::env;

use bb8::Pool;
use bb8_redis::RedisConnectionManager;

pub type RedisPool = Pool<RedisConnectionManager>;

pub async fn get_redis_pool() -> RedisPool {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    // Create a connection manager
    let manager =
        RedisConnectionManager::new(redis_url).expect("Failed to create Redis connection manager");

    // Create the pool
    let pool = Pool::builder()
        .build(manager)
        .await
        .expect("Failed to create Redis pool");

    pool
}
