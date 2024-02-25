use crate::{
    api::validation::validate_req_data,
    auth::user::get_user_id,
    services::postgres::{reservations::ReservationDb, DbPool},
    error::ServiceError,
};
use actix_web::{post, web, HttpResponse};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicReservation;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize, Validate)]
pub struct CreateReservationRequest {
    #[validate(range(min = 1))]
    pub space_id: i32,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
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

    let reservation = ReservationDb::insert(
        &pool,
        user_id,
        data.space_id,
        data.start_date,
        data.end_date,
    )
    .await?;
    Ok(HttpResponse::Created().json(CreateReservationResponse {
        reservation: reservation.into(),
    }))
}
