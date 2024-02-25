use crate::{
    auth::user::get_user_id,
    services::postgres::{stripe_customers::StripeCustomerDb, DbPool},
    error::ServiceError,
};
use actix_web::{post, web, HttpRequest, HttpResponse};
use serde::Deserialize;
use validator::Validate;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCustomerRequest {
    #[validate(length(min = 1))]
    pub customer_id: String,
}

// ======================================================================
// Route

#[post("")]
pub async fn create_customer(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    data: web::Json<CreateCustomerRequest>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;

    StripeCustomerDb::insert(&pool, user_id, &data.customer_id).await?;

    Ok(HttpResponse::Created().finish())
}
