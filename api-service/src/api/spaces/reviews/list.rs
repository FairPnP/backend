use crate::{
    api::validation::validate_req_data,
    error::ServiceError,
    services::postgres::{spaces::reviews::SpaceReviewDb, DbPool},
    utils::hashids::{decode_id, decode_id_option},
};
use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicSpaceReview;

// ======================================================================
// DTOs

#[derive(Deserialize, Validate)]
pub struct PaginationParams {
    #[validate(length(min = 10))]
    space_id: String,
    #[validate(length(min = 10))]
    offset_id: Option<String>,
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
    let space_id = decode_id(&query.space_id)?;

    let limit = query.limit.unwrap_or(10);
    let offset_id = decode_id_option(&query.offset_id)?;

    let space_reviews = SpaceReviewDb::list(&pool, space_id, limit, offset_id).await?;

    Ok(HttpResponse::Ok().json(ListSpaceReviewsResponse {
        space_reviews: space_reviews
            .into_iter()
            .map(PublicSpaceReview::from)
            .collect(),
    }))
}
