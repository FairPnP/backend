use crate::{db::DbPool, error::ServiceError};
use actix_web::{delete, web, HttpResponse};

// ======================================================================
// Route

#[delete("/{id}")]
pub async fn delete_building(
    pool: web::Data<DbPool>,
    building_id: web::Path<i32>,
) -> Result<HttpResponse, ServiceError> {
    delete_building_by_id(&pool, building_id.into_inner()).await?;
    Ok(HttpResponse::Ok().finish())
}

// ======================================================================
// Database operations

async fn delete_building_by_id(pool: &DbPool, building_id: i32) -> Result<(), ServiceError> {
    sqlx::query("DELETE FROM buildings WHERE id = $1")
        .bind(building_id)
        .execute(pool)
        .await?;
    Ok(())
}
