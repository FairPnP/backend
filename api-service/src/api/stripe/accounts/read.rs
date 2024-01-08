use crate::{
    auth::user::get_user_id,
    db::{stripe_accounts::StripeAccountDb, DbPool},
    error::ServiceError,
};
use actix_web::{get, web, HttpRequest, HttpResponse};
use serde::Serialize;

// ======================================================================
// DTOs

#[derive(Debug, Serialize)]
pub struct ReadAccountResponse {
    account_id: String,
}

// ======================================================================
// Route

#[get("")]
pub async fn read_account(
    req: HttpRequest,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;

    let account = StripeAccountDb::get(&pool, user_id).await?;

    Ok(HttpResponse::Ok().json(ReadAccountResponse {
        account_id: account.account_id,
    }))
}
