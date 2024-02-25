use crate::{
    auth::user::get_user_id,
    services::postgres::{users::reviews::UserReviewDb, DbPool},
    error::ServiceError,
};
use actix_web::{delete, web, HttpResponse};
use uuid::Uuid;

// ======================================================================
// Route

#[delete("/{id}")]
pub async fn delete_user_review(
    pool: web::Data<DbPool>,
    req: actix_web::HttpRequest,
    to_user_id: web::Path<String>,
) -> Result<HttpResponse, ServiceError> {
    let from_user_id = get_user_id(&req)?;
    let to_user_id = Uuid::parse_str(&to_user_id)
        .map_err(|_| ServiceError::BadRequest("Invalid user_id".into()))?;

    UserReviewDb::delete(&pool, from_user_id, to_user_id).await?;
    Ok(HttpResponse::Ok().finish())
}
