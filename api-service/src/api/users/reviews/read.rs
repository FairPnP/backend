use crate::{
    auth::user::get_user_id,
    db::{users::reviews::UserReviewDb, DbPool},
    error::ServiceError,
};
use actix_web::{get, web, HttpRequest, HttpResponse};
use serde::Serialize;
use uuid::Uuid;

use super::public::PublicUserReview;

// ======================================================================
// DTOs

#[derive(Debug, Serialize)]
pub struct ReadUserReviewResponse {
    user_review: PublicUserReview,
}

// ======================================================================
// Route

#[get("/{id}")]
pub async fn read_user_review(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    user_id: web::Path<String>,
) -> Result<HttpResponse, ServiceError> {
    let from_user_id = get_user_id(&req)?;
    let to_user_id = Uuid::parse_str(&user_id)
        .map_err(|_| ServiceError::BadRequest("Invalid user_id".into()))?;

    let user_review = UserReviewDb::get(&pool, from_user_id, to_user_id).await?;
    Ok(HttpResponse::Ok().json(ReadUserReviewResponse {
        user_review: user_review.into(),
    }))
}
