use crate::{
    auth::user::get_user_id,
    db::{stripe_accounts::StripeAccountDb, DbPool},
    error::ServiceError,
    stripe::StripeClient,
};
use actix_web::{post, web, HttpRequest, HttpResponse};
use serde::Serialize;

// ======================================================================
// DTOs

#[derive(Debug, Serialize)]
pub struct CreateAccountResponse {
    pub link: String,
}

// ======================================================================
// Route

#[post("")]
pub async fn create_account(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    stripe_client: web::Data<StripeClient>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let account_id = stripe_client.create_account().await?;
    StripeAccountDb::insert(&pool, user_id, &account_id).await?;
    let link = stripe_client.create_account_link(&account_id).await?;

    Ok(HttpResponse::Created().json(CreateAccountResponse { link }))
}
