use crate::{
    api::validation::validate_req_data,
    error::ServiceError,
    services::postgres::{users::reviews::UserReviewDb, DbPool},
    utils::hashids::decode_id_option,
};
use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::public::PublicUserReview;

// ======================================================================
// DTOs

#[derive(Deserialize, Validate)]
pub struct PaginationParams {
    user_id: String,
    #[validate(length(min = 10))]
    offset_id: Option<String>,
    #[validate(range(min = 1))]
    limit: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ListUserReviewsResponse {
    pub user_reviews: Vec<PublicUserReview>,
}

// ======================================================================
// Route

#[get("")]
pub async fn list_user_reviews(
    pool: web::Data<DbPool>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, ServiceError> {
    let query = validate_req_data(query.into_inner())?;
    let to_user_id = Uuid::parse_str(&query.user_id)
        .map_err(|_| ServiceError::BadRequest("Invalid user_id".into()))?;

    let limit = query.limit.unwrap_or(10);
    let offset_id = decode_id_option(&query.offset_id)?;
    let user_reviews = UserReviewDb::list(&pool, to_user_id, limit, offset_id).await?;

    Ok(HttpResponse::Ok().json(ListUserReviewsResponse {
        user_reviews: user_reviews
            .into_iter()
            .map(PublicUserReview::from)
            .collect(),
    }))
}
