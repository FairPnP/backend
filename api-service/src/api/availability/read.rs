use crate::{
    db::{availability::AvailabilityDb, DbPool},
    error::ServiceError,
};
use actix_web::{get, web, HttpResponse};
use serde::Serialize;

use super::public::PublicAvailability;

// ======================================================================
// DTOs

#[derive(Debug, Serialize)]
pub struct ReadAvailabilityResponse {
    availability: PublicAvailability,
}

// ======================================================================
// Route

#[get("/{id}")]
pub async fn read_availability(
    pool: web::Data<DbPool>,
    availability_id: web::Path<i32>,
) -> Result<HttpResponse, ServiceError> {
    let availability = AvailabilityDb::get(&pool, availability_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ReadAvailabilityResponse {
        availability: availability.into(),
    }))
}
