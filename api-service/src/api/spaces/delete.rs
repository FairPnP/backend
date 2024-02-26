use crate::{
    auth::user::get_user_id,
    error::ServiceError,
    services::postgres::{spaces::SpaceDb, DbPool},
    utils::hashids::decode_id,
};
use actix_web::{delete, web, HttpResponse};

// ======================================================================
// Route

#[delete("/{id}")]
pub async fn delete_space(
    pool: web::Data<DbPool>,
    req: actix_web::HttpRequest,
    space_id: web::Path<String>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let space_id = decode_id(&space_id.into_inner())?;

    SpaceDb::delete(&pool, user_id, space_id).await?;
    Ok(HttpResponse::Ok().finish())
}
