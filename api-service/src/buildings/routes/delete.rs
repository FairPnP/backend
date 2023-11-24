use crate::db::get_db_connection;
use crate::schema::boards::dsl;
use crate::{db::DbPool, error::ServiceError};
use actix_web::{delete, web, HttpResponse};
use diesel::prelude::*;

// ======================================================================
// Route

#[delete("/{id}")]
pub async fn delete_board(
    pool: web::Data<DbPool>,
    board_id: web::Path<i32>,
) -> Result<HttpResponse, ServiceError> {
    delete_board_by_id(&pool, board_id.into_inner())?;
    Ok(HttpResponse::Ok().finish())
}

// ======================================================================
// Database operations

fn delete_board_by_id(pool: &DbPool, board_id: i32) -> Result<usize, ServiceError> {
    let mut conn = get_db_connection(pool)?;
    diesel::delete(dsl::boards.find(board_id))
        .execute(&mut conn)
        .map_err(From::from)
}
