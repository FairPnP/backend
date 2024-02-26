use crate::{
    api::validation::validate_req_data,
    auth::user::get_user_id,
    error::ServiceError,
    services::postgres::{
        reservation_chat_messages::ReservationChatMessageDb, reservations::ReservationDb, DbPool,
    },
    utils::hashids::{decode_id, decode_id_option, encode_id},
};
use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicChatMessage;

// ======================================================================
// DTOs

#[derive(Deserialize, Validate)]
pub struct PaginationParams {
    #[validate(length(min = 10))]
    before_id: Option<String>,
    #[validate(length(min = 10))]
    after_id: Option<String>,
    #[validate(range(min = 1))]
    limit: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ListChatMessagesResponse {
    pub messages: Vec<PublicChatMessage>,
    pub reservation_id: String,
    pub next_offset_id: Option<i32>,
    pub limit: i32,
}

// ======================================================================
// Route

#[get("{reservation_id}")]
pub async fn list_chat_messages(
    req: actix_web::HttpRequest,
    pool: web::Data<DbPool>,
    query: web::Query<PaginationParams>,
    reservation_id: web::Path<String>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let query = validate_req_data(query.into_inner())?;
    let reservation_id = decode_id(&reservation_id.into_inner())?;

    // check if user is authorized to view this reservation
    let reservation = ReservationDb::get(&pool, reservation_id).await?;
    if reservation.user_id != user_id {
        let space =
            crate::services::postgres::spaces::SpaceDb::get(&pool, reservation.space_id).await?;
        if space.user_id != user_id {
            return Err(ServiceError::Unauthorized);
        }
    }

    // limit default to 20, max 50
    let limit = query.limit.map_or(20, |l| if l > 50 { 50 } else { l });
    let messages = ReservationChatMessageDb::list_messages(
        &pool,
        reservation_id,
        decode_id_option(&query.before_id)?,
        decode_id_option(&query.after_id)?,
        limit,
    )
    .await?;

    let messages = messages.into_iter().map(PublicChatMessage::from).collect();

    Ok(HttpResponse::Ok().json(ListChatMessagesResponse {
        messages,
        reservation_id: encode_id(reservation_id),
        next_offset_id: None,
        limit,
    }))
}
