use crate::{
    api::validation::validate_req_data,
    auth::user::get_user_id,
    error::ServiceError,
    services::postgres::{spaces::reviews::SpaceReviewDb, DbPool},
    utils::hashids::decode_id,
};
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicSpaceReview;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize, Validate)]
pub struct CreateSpaceReviewRequest {
    #[validate(length(min = 10))]
    pub space_id: String,
    #[validate(length(min = 1))]
    pub message: String,
    #[validate(range(min = 1, max = 5))]
    pub stars: i32,
}

#[derive(Debug, Serialize)]
pub struct CreateSpaceReviewResponse {
    pub space_review: PublicSpaceReview,
}

// ======================================================================
// Route

#[post("")]
pub async fn create_space_review(
    req: actix_web::HttpRequest,
    pool: web::Data<DbPool>,
    data: web::Json<CreateSpaceReviewRequest>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let data = validate_req_data(data.into_inner())?;
    let space_id = decode_id(&data.space_id)?;

    let space_review =
        SpaceReviewDb::insert(&pool, user_id, space_id, data.message, data.stars).await?;
    Ok(HttpResponse::Created().json(CreateSpaceReviewResponse {
        space_review: space_review.into(),
    }))
}
