use actix_web::{get, HttpResponse};

use crate::error::ServiceError;

#[get("/stripe/return")]
pub async fn stripe_return() -> Result<HttpResponse, ServiceError> {
    let return_url = std::env::var("STRIPE_RETURN_URL").expect("Missing STRIPE_RETURN_URL in env");

    // redirect to return_url
    Ok(HttpResponse::Found()
        .append_header(("Location", return_url))
        .finish())
}

#[get("/stripe/refresh")]
pub async fn stripe_refresh() -> Result<HttpResponse, ServiceError> {
    let refresh_url =
        std::env::var("STRIPE_REFRESH_URL").expect("Missing STRIPE_REFRESH_URL in env");

    // redirect to refresh_url
    Ok(HttpResponse::Found()
        .append_header(("Location", refresh_url))
        .finish())
}
