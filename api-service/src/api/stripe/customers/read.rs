use crate::{
    auth::user::get_user_id,
    db::{stripe_customers::StripeCustomerDb, DbPool},
    error::ServiceError,
};
use actix_web::{get, web, HttpRequest, HttpResponse};
use serde::Serialize;

// ======================================================================
// DTOs

#[derive(Debug, Serialize)]
pub struct ReadCustomerResponse {
    customer_id: String,
}

// ======================================================================
// Route

#[get("")]
pub async fn read_customer(
    req: HttpRequest,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;

    let customer = StripeCustomerDb::get(&pool, user_id).await?;

    Ok(HttpResponse::Ok().json(ReadCustomerResponse {
        customer_id: customer.customer_id,
    }))
}
