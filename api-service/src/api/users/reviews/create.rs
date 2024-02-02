use crate::{
    api::validation::validate_req_data,
    auth::user::get_user_id,
    db::{users::reviews::UserReviewDb, DbPool},
    error::ServiceError,
};
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::public::PublicUserReview;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserReviewRequest {
    #[validate(length(min = 1))]
    pub user_id: String,
    #[validate(length(min = 1))]
    pub message: String,
    #[validate(range(min = 1, max = 5))]
    pub stars: i32,
}

#[derive(Debug, Serialize)]
pub struct CreateUserReviewResponse {
    pub user_review: PublicUserReview,
}

// ======================================================================
// Route

#[post("")]
pub async fn create_user_review(
    req: actix_web::HttpRequest,
    pool: web::Data<DbPool>,
    data: web::Json<CreateUserReviewRequest>,
) -> Result<HttpResponse, ServiceError> {
    let from_user_id = get_user_id(&req)?;
    let data = validate_req_data(data.into_inner())?;
    let to_user_id = Uuid::parse_str(&data.user_id)
        .map_err(|_| ServiceError::BadRequest("Invalid user_id".into()))?;

    let user_review =
        UserReviewDb::insert(&pool, from_user_id, to_user_id, data.message, data.stars).await?;
    Ok(HttpResponse::Created().json(CreateUserReviewResponse {
        user_review: user_review.into(),
    }))
}
