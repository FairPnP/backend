use crate::{
    error::ServiceError,
    services::postgres::{availability::AvailabilityDb, DbPool},
    utils::hashids::decode_id,
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
    availability_id: web::Path<String>,
) -> Result<HttpResponse, ServiceError> {
    let availability_id = decode_id(&availability_id.into_inner())?;
    let availability = AvailabilityDb::get(&pool, availability_id).await?;
    Ok(HttpResponse::Ok().json(ReadAvailabilityResponse {
        availability: availability.into(),
    }))
}
