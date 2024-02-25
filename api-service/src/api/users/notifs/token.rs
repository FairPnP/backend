use crate::{
    api::validation::validate_req_data, auth::user::get_user_id, error::ServiceError,
    services::postgres::users::notif_tokens::UserNotifTokenDb,
};
use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::services::postgres::DbPool;

use super::public::PublicUserNotifToken;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserNotifTokenRequest {
    #[validate(length(min = 1))]
    pub expo_token: String,
    #[validate(length(min = 1))]
    pub device_token: String,
    #[validate(length(min = 1))]
    pub device_type: String,
}

#[derive(Debug, Serialize, Validate)]
pub struct ListUserNotifTokenResponse {
    pub tokens: Vec<PublicUserNotifToken>,
}

// ======================================================================
// Route

#[post("token")]
pub async fn update_token(
    req: actix_web::HttpRequest,
    pool: web::Data<DbPool>,
    data: web::Json<CreateUserNotifTokenRequest>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let data = validate_req_data(data.into_inner())?;

    UserNotifTokenDb::upsert(
        &pool,
        user_id,
        Some(data.expo_token),
        data.device_token,
        data.device_type,
    )
    .await?;

    Ok(HttpResponse::Created().finish())
}

#[get("token")]
pub async fn list_tokens(
    req: actix_web::HttpRequest,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let tokens = UserNotifTokenDb::list(&pool, user_id).await?;

    Ok(HttpResponse::Ok().json(ListUserNotifTokenResponse {
        tokens: tokens.into_iter().map(|t| t.into()).collect(),
    }))
}
