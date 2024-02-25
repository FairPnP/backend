use crate::{
    api::validation::validate_req_data,
    auth::user::get_user_id,
    error::ServiceError,
    services::postgres::{availability::AvailabilityDb, DbPool},
};
use actix_web::{post, web, HttpResponse};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicAvailability;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize, Validate)]
pub struct CreateAvailabilityRequest {
    pub space_id: i32,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub hourly_rate: BigDecimal,
}

#[derive(Debug, Serialize)]
pub struct CreateAvailabilityResponse {
    pub availability: PublicAvailability,
}

// ======================================================================
// Route

#[post("")]
pub async fn create_availability(
    req: actix_web::HttpRequest,
    pool: web::Data<DbPool>,
    data: web::Json<CreateAvailabilityRequest>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let data = validate_req_data(data.into_inner())?;

    let availability = AvailabilityDb::insert(
        &pool,
        user_id,
        data.space_id,
        data.start_date,
        data.end_date,
        data.hourly_rate,
    )
    .await?;
    Ok(HttpResponse::Created().json(CreateAvailabilityResponse {
        availability: availability.into(),
    }))
}
