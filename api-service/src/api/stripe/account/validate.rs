use crate::{
    auth::user::get_user_id,
    db::{stripe_accounts::StripeAccountDb, DbPool},
    error::ServiceError,
    stripe::{
        account::{service::Account, types::AccountId},
        client::StripeClient,
    },
};
use actix_web::{get, web, HttpRequest, HttpResponse};
use serde::Serialize;

// ======================================================================
// DTOs

#[derive(Debug, Serialize)]
pub struct ValidateAccountResponse {
    pub is_valid: bool,
}

// ======================================================================
// Route

#[get("validate")]
pub async fn validate_account(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    stripe_client: web::Data<StripeClient>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;

    let account = StripeAccountDb::get(&pool, user_id).await;

    let mut is_valid = false;
    if let Ok(account) = account {
        let account_id = AccountId::new(account.account_id);
        let stripe_account = Account::get_account(&stripe_client, &account_id).await;
        if let Ok(stripe_account) = stripe_account {
            if let Some(details_submitted) = stripe_account.details_submitted {
                is_valid = details_submitted;
            }
        }
    }

    Ok(HttpResponse::Ok().json(ValidateAccountResponse { is_valid }))
}
