use crate::{
    api::validation::validate_req_data,
    auth::user::get_user_id,
    db::{availability::AvailabilityDb, DbPool},
    error::ServiceError,
};
use actix_web::{put, web, HttpResponse};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicAvailability;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateAvailabilityRequest {
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub hourly_rate: Option<BigDecimal>,
}

#[derive(Debug, Serialize)]
pub struct UpdateAvailabilityResponse {
    availability: PublicAvailability,
}

// ======================================================================
// Route

#[put("/{id}")]
pub async fn update_availability(
    pool: web::Data<DbPool>,
    req: actix_web::HttpRequest,
    availability_id: web::Path<i32>,
    data: web::Json<UpdateAvailabilityRequest>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let data = validate_req_data(data.into_inner())?;
    let availability_id = availability_id.into_inner();

    let updated_availability = AvailabilityDb::update(
        &pool,
        user_id,
        availability_id,
        data.start_date.to_owned(),
        data.end_date.to_owned(),
        data.hourly_rate,
    )
    .await?;
    Ok(HttpResponse::Ok().json(UpdateAvailabilityResponse {
        availability: updated_availability.into(),
    }))
}
