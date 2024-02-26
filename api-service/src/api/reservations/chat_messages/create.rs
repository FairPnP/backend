use crate::{
    auth::user::get_user_id,
    error::ServiceError,
    services::{
        expo::send_push_notification,
        postgres::{
            reservation_chat_messages::ReservationChatMessageDb,
            reservations::ReservationDb,
            spaces::SpaceDb,
            users::{notif_tokens::UserNotifTokenDb, profiles::UserProfileDb},
            DbPool,
        },
    },
    utils::hashids::{decode_id, encode_id},
};
use actix_web::{post, web, HttpResponse};
use expo_push_notification_client::Expo;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicChatMessage;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize, Validate)]
pub struct CreateChatMessageRequest {
    #[validate(length(min = 10))]
    pub reservation_id: String,
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
    expo: web::Data<Expo>,
    data: web::Json<CreateChatMessageRequest>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let reservation_id = decode_id(&data.reservation_id)?;

    let reservation = ReservationDb::get(&pool, reservation_id).await?;
    let space = SpaceDb::get(&pool, reservation.space_id).await?;
    // check if user is part of reservation or space
    if !(reservation.user_id == user_id || space.user_id == user_id) {
        return Err(ServiceError::Unauthorized);
    }

    let chat_message =
        ReservationChatMessageDb::insert(&pool, reservation_id, user_id, data.message.clone())
            .await?;

    let other_user_id = if reservation.user_id == user_id {
        space.user_id
    } else {
        reservation.user_id
    };
    let tokens = UserNotifTokenDb::list(&pool, other_user_id).await?;
    // get valid expo tokens, and dedupe
    let tokens = tokens
        .into_iter()
        .filter_map(|t| t.expo_token)
        .collect::<Vec<String>>();

    if !tokens.is_empty() {
        let user_profile = UserProfileDb::get(&pool, user_id).await;
        let user_profile = match user_profile {
            Ok(user) => user.name,
            Err(_) => "User".to_string(),
        };

        let message_data = MessageNotifData {
            screen_name: "ReservationChat".to_string(),
            screen_params: ChatScreenParams {
                reservation_id: encode_id(reservation_id),
            },
        };

        let res = send_push_notification(
            &expo,
            tokens,
            user_profile,
            data.message.clone(),
            &message_data,
        )
        .await;

        match res {
            Ok(tickets) => println!("tickets: {:?}", tickets),
            Err(_) => println!("Failed to send push notification, {:?}", user_id),
        }
    }

    Ok(HttpResponse::Created().json(CreateChatMessageResponse {
        message: chat_message.into(),
    }))
}

#[derive(Serialize)]
struct ChatScreenParams {
    reservation_id: String,
}

#[derive(Serialize)]
struct MessageNotifData {
    screen_name: String,
    screen_params: ChatScreenParams,
}
