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
use actix_web::{post, web, HttpRequest, HttpResponse};
use serde::Serialize;

// ======================================================================
// DTOs

#[derive(Debug, Serialize)]
pub struct DashboardResponse {
    pub link: String,
}

// ======================================================================
// Route

#[post("/dashboard")]
pub async fn dashboard(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    stripe_client: web::Data<StripeClient>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;

    // check if account already exists
    let account = StripeAccountDb::get(&pool, user_id).await;
    let account_id = match account {
        Ok(account) => AccountId::new(account.account_id),
        Err(sqlx::Error::RowNotFound) => {
            // account doesn't exist, create it
            let account = Account::create_account(&stripe_client).await?;
            StripeAccountDb::insert(&pool, user_id, account.id.as_str()).await?;

            account.id
        }
        Err(err) => return Err(err.into()),
    };

    // check if account is already verified
    let is_onboarding_complete =
        Account::is_onboarding_complete(&stripe_client, &account_id).await?;

    // create link
    let link = match is_onboarding_complete {
        true => {
            Account::create_login_link(&stripe_client, &account_id)
                .await?
                .url
        }
        false => {
            Account::create_account_link(&stripe_client, &account_id)
                .await?
                .url
        }
    };

    Ok(HttpResponse::Ok().json(DashboardResponse { link }))
}
