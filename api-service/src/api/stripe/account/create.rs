use crate::{
    auth::user::get_user_id,
    db::{stripe_accounts::StripeAccountDb, DbPool},
    error::ServiceError,
    stripe::account::StripeAccount,
};
use actix_web::{post, web, HttpRequest, HttpResponse};
use serde::Serialize;
use stripe::Client;

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
    client: web::Data<Client>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let account = StripeAccount::create_account(&client).await?;
    StripeAccountDb::insert(&pool, user_id, account.id.to_string()).await?;
    let link = StripeAccount::create_account_link(&client, account.id).await?;

    Ok(HttpResponse::Created().json(CreateAccountResponse { link: link.url }))
}
