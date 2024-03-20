use actix_cors::Cors;
use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use google_maps::prelude::*;
use services::{
    expo::get_expo_client, postgres::establish_connection, redis::get_redis_pool, s3::get_s3_client,
};
use tracing::Level;
use tracing_actix_web::TracingLogger;

mod api;
mod auth;
mod error;
mod health;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let postgres_pool = establish_connection().await;
    let redis_pool = get_redis_pool().await;
    let s3_client = get_s3_client();
    let expo_client = get_expo_client();

    let google_api_key = std::env::var("GOOGLE_API_KEY").expect("Missing GOOGLE_API_KEY in env");
    let google_maps_client = GoogleMapsClient::try_new(&google_api_key).unwrap();

    let server_bind_address = format!(
        "0.0.0.0:{port}",
        port = std::env::var("PORT").expect("Missing PORT in env")
    );

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .app_data(Data::new(postgres_pool.clone()))
            .app_data(Data::new(redis_pool.clone()))
            .app_data(Data::new(s3_client.clone()))
            .app_data(Data::new(expo_client.clone()))
            .app_data(Data::new(google_maps_client.clone()))
            .configure(health::config)
            .service(web::scope("/api").configure(api::config))
    })
    .bind(server_bind_address)?
    .run()
    .await
}
