use crate::{
    auth::user::get_user_id,
    db::{reservation_chat_messages::ReservationChatMessageDb, DbPool},
    error::ServiceError,
};
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicChatMessage;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize, Validate)]
pub struct CreateChatMessageRequest {
    #[validate(range(min = 1))]
    pub reservation_id: i32,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct CreateChatMessageResponse {
    pub message: PublicChatMessage,
}

// ======================================================================
// Route

#[post("")]
pub async fn create_chat_message(
    req: actix_web::HttpRequest,
    pool: web::Data<DbPool>,
    data: web::Json<CreateChatMessageRequest>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;

    let chat_message =
        ReservationChatMessageDb::insert(&pool, data.reservation_id, user_id, data.message.clone())
            .await?;

    Ok(HttpResponse::Created().json(CreateChatMessageResponse {
        message: chat_message.into(),
    }))
}
