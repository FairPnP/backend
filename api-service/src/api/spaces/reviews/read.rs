use crate::{
    db::{spaces::reviews::SpaceReviewDb, DbPool},
    error::ServiceError,
};
use actix_web::{get, web, HttpResponse};
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
    pool: web::Data<DbPool>,
    space_review_id: web::Path<i32>,
) -> Result<HttpResponse, ServiceError> {
    let space_review = SpaceReviewDb::get(&pool, *space_review_id).await?;
    Ok(HttpResponse::Ok().json(ReadSpaceReviewResponse {
        space_review: space_review.into(),
    }))
}
