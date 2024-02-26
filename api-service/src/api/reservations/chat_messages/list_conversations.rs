use crate::{
    api::validation::validate_req_data,
    auth::user::get_user_id,
    error::ServiceError,
    services::postgres::{reservation_chat_messages::ReservationChatMessageDb, DbPool},
    utils::hashids::{decode_id_option, encode_id},
};
use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicConversationSummary;

// ======================================================================
// DTOs

#[derive(Deserialize, Validate)]
pub struct PaginationParams {
    #[validate(length(min = 10))]
    offset_id: Option<String>,
    #[validate(range(min = 1))]
    limit: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ListConversationsResponse {
    pub conversations: Vec<PublicConversationSummary>,
    pub next_offset_id: Option<String>,
    pub limit: i32,
}

// ======================================================================
// Route

#[get("/host")]
pub async fn list_host_conversations(
    req: actix_web::HttpRequest,
    pool: web::Data<DbPool>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, ServiceError> {
    let host_user_id = get_user_id(&req)?;
    let query = validate_req_data(query.into_inner())?;
    let limit = query.limit.map_or(10, |l| if l > 20 { 20 } else { l });
    let offset_id = decode_id_option(&query.offset_id)?;

    let conversations = ReservationChatMessageDb::list_conversations_for_host(
        &pool,
        host_user_id,
        offset_id,
        limit,
    )
    .await?;

    let next_offset_id = if conversations.len() as i32 == limit {
        conversations.last().map(|b| encode_id(b.reservation_id))
    } else {
        None
    };

    Ok(HttpResponse::Ok().json(ListConversationsResponse {
        conversations: conversations
            .into_iter()
            .map(PublicConversationSummary::from)
            .collect::<Vec<_>>(),
        limit,
        next_offset_id,
    }))
}

#[get("/guest")]
pub async fn list_guest_conversations(
    req: actix_web::HttpRequest,
    pool: web::Data<DbPool>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, ServiceError> {
    let guest_user_id = get_user_id(&req)?;
    let query = validate_req_data(query.into_inner())?;
    let limit = query.limit.map_or(10, |l| if l > 20 { 20 } else { l });
    let offset_id = decode_id_option(&query.offset_id)?;

    let conversations = ReservationChatMessageDb::list_conversations_for_guest(
        &pool,
        guest_user_id,
        offset_id,
        limit,
    )
    .await?;
    let next_offset_id = if conversations.len() as i32 == limit {
        conversations.last().map(|b| encode_id(b.reservation_id))
    } else {
        None
    };

    Ok(HttpResponse::Ok().json(ListConversationsResponse {
        conversations: conversations
            .into_iter()
            .map(PublicConversationSummary::from)
            .collect::<Vec<_>>(),
        limit,
        next_offset_id,
    }))
}
