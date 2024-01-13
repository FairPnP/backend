use crate::{
    auth::user::get_user_id,
    db::{stripe_customers::StripeCustomerDb, DbPool},
    error::ServiceError,
    stripe::{
        account::types::AccountId,
        client::StripeClient,
        customer::{service::Customer, types::CustomerId},
    },
};
use actix_web::{post, web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePaymentRequest {
    pub dest_account: AccountId,
    pub amount: i64,
}

#[derive(Debug, Serialize)]
pub struct CreatePaymentResponse {
    customer_id: String,
    client_secret: String,
    ephemeral_key: String,
}

// ======================================================================
// Route

#[post("")]
pub async fn create_payment(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    stripe_client: web::Data<StripeClient>,
    data: web::Json<CreatePaymentRequest>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;

    let customer = StripeCustomerDb::get(&pool, user_id).await;
    // if no customer, create one
    let customer = match customer {
        Ok(customer) => customer,
        Err(sqlx::Error::RowNotFound) => {
            let s_customer = Customer::create_customer(&stripe_client).await?;
            let customer = StripeCustomerDb::insert(&pool, user_id, s_customer.id.as_str()).await?;

            customer
        }
        Err(err) => return Err(err.into()),
    };

    let customer_id = CustomerId::new(customer.customer_id);
    let ephemeral_key = Customer::create_ephemeral_key(&stripe_client, &customer_id).await?;

    // create payment intent
    let amount = data.amount;
    let currency = "cad";
    let platform_fee = (amount as f64 * 0.1).round() as i64;
    let payment_intent = Customer::create_payment_intent(
        &stripe_client,
        &customer_id,
        &data.dest_account,
        amount,
        currency,
        platform_fee,
    )
    .await?;

    let response = CreatePaymentResponse {
        customer_id: customer_id.as_str().to_string(),
        client_secret: payment_intent.client_secret,
        ephemeral_key: ephemeral_key.secret.unwrap_or("".to_string()),
    };

    Ok(HttpResponse::Created().json(response))
}
