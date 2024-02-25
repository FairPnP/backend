use crate::{
    api::validation::validate_req_data,
    auth::user::get_user_id,
    services::postgres::{spaces::reviews::SpaceReviewDb, DbPool},
    error::ServiceError,
};
use actix_web::{put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicSpaceReview;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateSpaceReviewRequest {
    #[validate(length(min = 1))]
    pub message: Option<String>,
    #[validate(range(min = 1, max = 5))]
    pub stars: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct UpdateSpaceReviewResponse {
    space_review: PublicSpaceReview,
}

// ======================================================================
// Route

#[put("/{id}")]
pub async fn update_space_review(
    pool: web::Data<DbPool>,
    space_id: web::Path<i32>,
    req: actix_web::HttpRequest,
    data: web::Json<UpdateSpaceReviewRequest>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let data = validate_req_data(data.into_inner())?;
    let space_id = space_id.into_inner();

    let updated_space_review =
        SpaceReviewDb::update(&pool, user_id, space_id, data.message, data.stars).await?;
    Ok(HttpResponse::Ok().json(UpdateSpaceReviewResponse {
        space_review: updated_space_review.into(),
    }))
}
