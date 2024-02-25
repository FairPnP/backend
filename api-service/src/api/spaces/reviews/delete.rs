use crate::{
    auth::user::get_user_id,
    services::postgres::{spaces::reviews::SpaceReviewDb, DbPool},
    error::ServiceError,
};
use actix_web::{delete, web, HttpResponse};

// ======================================================================
// Route

#[delete("/{id}")]
pub async fn delete_space_review(
    pool: web::Data<DbPool>,
    req: actix_web::HttpRequest,
    space_review_id: web::Path<i32>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;

    SpaceReviewDb::delete(&pool, user_id, *space_review_id).await?;
    Ok(HttpResponse::Ok().finish())
}
