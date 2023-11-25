use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use auth::{middleware::jwt_validator, state::JwtValidatorState};
use db::establish_connection;
use tracing::Level;
use tracing_actix_web::TracingLogger;
use tracing_subscriber;

mod auth;
mod buildings;
mod db;
mod dev;
mod error;
mod health;
mod schema;
mod users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let pool = establish_connection();
    let jwt_issuer = std::env::var("AUTH_ISSUER").expect("AUTH_ISSUER must be set");
    let jwks_uri = std::env::var("AUTH_JWKS_URL").expect("AUTH_JWKS_URL must be set");
    let jwt_validator_state = JwtValidatorState::new(jwt_issuer, jwks_uri);
    jwt_validator_state.get_jwks().await;
    let jwt_validator_state = Arc::new(jwt_validator_state);

    let server_bind_address =
        std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:3000".to_string());

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .app_data(Data::new(pool.clone()))
            .configure(health::config)
            .service(
                web::scope("/api")
                    .app_data(Data::new(jwt_validator_state.clone()))
                    .wrap(HttpAuthentication::bearer(jwt_validator))
                    .configure(buildings::routes::config)
                    .configure(dev::config),
            )
    })
    .bind(server_bind_address)?
    .run()
    .await
}
