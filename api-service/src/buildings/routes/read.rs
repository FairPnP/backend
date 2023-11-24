use crate::db::DbPool;
use crate::error::ServiceError;
use crate::schema::boards::dsl;
use crate::{board::entities::Board, db::get_db_connection};
use actix_web::{get, web, HttpResponse};
use diesel::prelude::*;
use serde::Serialize;

// ======================================================================
// DTOs

#[derive(Debug, Serialize)]
pub struct ReadBoardResponse {
    board: Board,
}

// ======================================================================
// Route

#[get("/{id}")]
pub async fn read_board(
    pool: web::Data<DbPool>,
    board_id: web::Path<i32>,
) -> Result<HttpResponse, ServiceError> {
    let board = get_board_by_id(&pool, board_id.into_inner())?;
    Ok(HttpResponse::Ok().json(ReadBoardResponse { board }))
}

// ======================================================================
// Database operations

fn get_board_by_id(pool: &DbPool, board_id: i32) -> Result<Board, ServiceError> {
    let mut conn = get_db_connection(pool)?;
    dsl::boards
        .find(board_id)
        .get_result(&mut conn)
        .map_err(From::from)
}
