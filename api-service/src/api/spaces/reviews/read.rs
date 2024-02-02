use crate::{
    auth::user::get_user_id,
    db::{spaces::reviews::SpaceReviewDb, DbPool},
    error::ServiceError,
};
use actix_web::{get, web, HttpRequest, HttpResponse};
use serde::Serialize;

use super::public::PublicSpaceReview;

// ======================================================================
// DTOs

#[derive(Debug, Serialize)]
pub struct ReadSpaceReviewResponse {
    space_review: PublicSpaceReview,
}

// ======================================================================
// Route

#[get("/{id}")]
pub async fn read_space_review(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    space_review_id: web::Path<i32>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;

    let space_review = SpaceReviewDb::get(&pool, user_id, *space_review_id).await?;
    Ok(HttpResponse::Ok().json(ReadSpaceReviewResponse {
        space_review: space_review.into(),
    }))
}
