use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use auth::{middleware::jwt_validator, state::JwtValidatorState};
use db::{establish_connection, redis::get_redis_pool, s3::get_s3_client};
use stripe::client::StripeClient;
use tracing::Level;
use tracing_actix_web::TracingLogger;
use tracing_subscriber;

mod api;
mod auth;
mod db;
mod error;
mod health;
mod redirect;
mod stripe;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let postgres_pool = establish_connection().await;
    let redis_pool = get_redis_pool().await;
    let jwt_issuer = std::env::var("AUTH_ISSUER").expect("AUTH_ISSUER must be set");
    let jwks_uri = std::env::var("AUTH_JWKS_URL").expect("AUTH_JWKS_URL must be set");
    let jwt_validator_state = JwtValidatorState::new(jwt_issuer, jwks_uri);
    jwt_validator_state.get_jwks().await;
    let jwt_validator_state = Arc::new(jwt_validator_state);
    let s3_client = get_s3_client();

    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let base_url = std::env::var("BASE_URL").expect("Missing BASE_URL in env");
    let return_url = format!("{}/redirect/stripe/return", base_url);
    let refresh_url = format!("{}/redirect/stripe/refresh", base_url);
    let stripe_client = StripeClient::new(secret_key, return_url, refresh_url);

    let server_bind_address =
        std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:3000".to_string());

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .app_data(Data::new(postgres_pool.clone()))
            .app_data(Data::new(redis_pool.clone()))
            .app_data(Data::new(s3_client.clone()))
            .app_data(Data::new(stripe_client.clone()))
            .configure(health::config)
            .configure(redirect::config)
            .service(
                web::scope("/api")
                    .app_data(Data::new(jwt_validator_state.clone()))
                    .wrap(HttpAuthentication::bearer(jwt_validator))
                    .configure(api::config),
            )
    })
    .bind(server_bind_address)?
    .run()
    .await
}
