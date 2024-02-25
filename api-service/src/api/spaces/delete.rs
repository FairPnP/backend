use crate::{
    auth::user::get_user_id,
    services::postgres::{spaces::SpaceDb, DbPool},
    error::ServiceError,
};
use actix_web::{delete, web, HttpResponse};

// ======================================================================
// Route

#[delete("/{id}")]
pub async fn delete_space(
    pool: web::Data<DbPool>,
    req: actix_web::HttpRequest,
    space_id: web::Path<i32>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;

    SpaceDb::delete(&pool, user_id, space_id.into_inner()).await?;
    Ok(HttpResponse::Ok().finish())
}
