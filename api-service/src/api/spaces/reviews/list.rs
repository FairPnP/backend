use crate::{
    api::validation::validate_req_data,
    services::postgres::{spaces::reviews::SpaceReviewDb, DbPool},
    error::ServiceError,
};
use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicSpaceReview;

// ======================================================================
// DTOs

#[derive(Deserialize, Validate)]
pub struct PaginationParams {
    space_id: i32,
    #[validate(range(min = 1))]
    offset_id: Option<i32>,
    #[validate(range(min = 1))]
    limit: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ListSpaceReviewsResponse {
    pub space_reviews: Vec<PublicSpaceReview>,
}

// ======================================================================
// Route

#[get("")]
pub async fn list_space_reviews(
    pool: web::Data<DbPool>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, ServiceError> {
    let query = validate_req_data(query.into_inner())?;

    let limit = query.limit.unwrap_or(10);

    let space_reviews = SpaceReviewDb::list(&pool, query.space_id, limit, query.offset_id).await?;

    Ok(HttpResponse::Ok().json(ListSpaceReviewsResponse {
        space_reviews: space_reviews
            .into_iter()
            .map(PublicSpaceReview::from)
            .collect(),
    }))
}
