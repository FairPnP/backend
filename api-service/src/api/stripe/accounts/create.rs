use crate::{
    auth::user::get_user_id,
    services::postgres::{stripe_accounts::StripeAccountDb, DbPool},
    error::ServiceError,
};
use actix_web::{post, web, HttpRequest, HttpResponse};
use serde::Deserialize;
use validator::Validate;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize, Validate)]
pub struct CreateAccountRequest {
    #[validate(length(min = 1))]
    pub account_id: String,
}

// ======================================================================
// Route

#[post("")]
pub async fn create_account(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    data: web::Json<CreateAccountRequest>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;

    StripeAccountDb::insert(&pool, user_id, &data.account_id).await?;

    Ok(HttpResponse::Created().finish())
}
