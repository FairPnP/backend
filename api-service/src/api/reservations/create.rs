use crate::{
    api::validation::validate_req_data,
    auth::user::get_user_id,
    error::ServiceError,
    services::postgres::{availability::AvailabilityDb, reservations::ReservationDb, DbPool},
    utils::hashids::decode_id,
};
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicReservation;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize, Validate)]
pub struct CreateReservationRequest {
    #[validate(length(min = 10))]
    pub availability_id: String,
}

#[derive(Debug, Serialize)]
pub struct CreateReservationResponse {
    pub reservation: PublicReservation,
}

// ======================================================================
// Route

#[post("")]
pub async fn create_reservation(
    req: actix_web::HttpRequest,
    pool: web::Data<DbPool>,
    data: web::Json<CreateReservationRequest>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let data = validate_req_data(data.into_inner())?;
    let availability_id = decode_id(&data.availability_id)?;

    let availability = AvailabilityDb::get(&pool, availability_id).await?;

    let space_id = availability.space_id;

    let reservation = ReservationDb::insert(
        &pool,
        user_id,
        space_id,
        availability_id,
        availability.start_date,
        availability.end_date,
    )
    .await?;
    Ok(HttpResponse::Created().json(CreateReservationResponse {
        reservation: reservation.into(),
    }))
}
