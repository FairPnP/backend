use crate::db::get_db_connection;
use crate::schema::buildings::dsl;
use crate::{db::DbPool, error::ServiceError};
use actix_web::{delete, web, HttpResponse};
use diesel::prelude::*;

// ======================================================================
// Route

#[delete("/{id}")]
pub async fn delete_building(
    pool: web::Data<DbPool>,
    building_id: web::Path<i32>,
) -> Result<HttpResponse, ServiceError> {
    delete_building_by_id(&pool, building_id.into_inner())?;
    Ok(HttpResponse::Ok().finish())
}

// ======================================================================
// Database operations

fn delete_building_by_id(pool: &DbPool, building_id: i32) -> Result<usize, ServiceError> {
    let mut conn = get_db_connection(pool)?;
    diesel::delete(dsl::buildings.find(building_id))
        .execute(&mut conn)
        .map_err(From::from)
}
