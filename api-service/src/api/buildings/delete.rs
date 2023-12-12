use crate::{
    db::{buildings::BuildingDb, DbPool},
    error::ServiceError,
};
use actix_web::{delete, web, HttpResponse};

// ======================================================================
// Route

#[delete("/{id}")]
pub async fn delete_building(
    pool: web::Data<DbPool>,
    building_id: web::Path<i32>,
) -> Result<HttpResponse, ServiceError> {
    BuildingDb::delete(&pool, building_id.into_inner()).await?;
    Ok(HttpResponse::Ok().finish())
}
