use crate::{
    auth::user::get_user_id,
    error::ServiceError,
    services::{
        postgres::{stripe_accounts::StripeAccountDb, DbPool},
        stripe::{
            account::{service::Account, types::AccountId},
            client::StripeClient,
        },
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

    // TODO: check onboarding status separately
    let mut is_valid = false;
    if let Ok(account) = account {
        let account_id = AccountId::new(account.account_id);
        let stripe_account = Account::get_account(&stripe_client, &account_id).await;
        if let Ok(stripe_account) = stripe_account {
            if let Some(charges_enabled) = stripe_account.charges_enabled {
                is_valid = charges_enabled;
            }
        }
    }

    Ok(HttpResponse::Ok().json(ValidateAccountResponse { is_valid }))
}
