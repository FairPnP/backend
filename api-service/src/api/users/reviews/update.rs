use crate::{
    api::validation::validate_req_data,
    auth::user::get_user_id,
    error::ServiceError,
    services::postgres::{users::reviews::UserReviewDb, DbPool},
};
use actix_web::{put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::public::PublicUserReview;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserReviewRequest {
    #[validate(length(min = 1))]
    pub message: Option<String>,
    #[validate(range(min = 1, max = 5))]
    pub stars: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct UpdateUserReviewResponse {
    user_review: PublicUserReview,
}

// ======================================================================
// Route

#[put("/{id}")]
pub async fn update_user_review(
    pool: web::Data<DbPool>,
    user_id: web::Path<String>,
    req: actix_web::HttpRequest,
    data: web::Json<UpdateUserReviewRequest>,
) -> Result<HttpResponse, ServiceError> {
    let from_user_id = get_user_id(&req)?;
    let data = validate_req_data(data.into_inner())?;
    let to_user_id = Uuid::parse_str(&user_id)
        .map_err(|_| ServiceError::BadRequest("Invalid user_id".into()))?;

    let updated_user_review =
        UserReviewDb::update(&pool, from_user_id, to_user_id, data.message, data.stars).await?;
    Ok(HttpResponse::Ok().json(UpdateUserReviewResponse {
        user_review: updated_user_review.into(),
    }))
}
