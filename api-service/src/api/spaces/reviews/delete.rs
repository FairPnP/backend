use crate::{
    auth::user::get_user_id,
    db::{
        spaces::{reviews::SpaceReviewDb, SpaceDb},
        DbPool,
    },
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

    let space_review = SpaceReviewDb::get(&pool, *space_review_id).await?;

    let space = SpaceDb::get(&pool, space_review.space_id).await?;
    // if user is not the owner of the space, return unauthorized
    if space.user_id != user_id {
        return Err(ServiceError::Unauthorized);
    }

    SpaceReviewDb::delete(&pool, *space_review_id).await?;
    Ok(HttpResponse::Ok().finish())
}
