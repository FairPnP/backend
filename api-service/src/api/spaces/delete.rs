use crate::{
    db::{spaces::SpaceDb, DbPool},
    error::ServiceError,
};
use actix_web::{delete, web, HttpResponse};

// ======================================================================
// Route

#[delete("/{id}")]
pub async fn delete_space(
    pool: web::Data<DbPool>,
    space_id: web::Path<i32>,
) -> Result<HttpResponse, ServiceError> {
    SpaceDb::delete(&pool, space_id.into_inner()).await?;
    Ok(HttpResponse::Ok().finish())
}
