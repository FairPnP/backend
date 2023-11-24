use crate::board::entities::{Board, UpdateBoard};
use crate::db::{get_db_connection, DbPool};
use crate::error::ServiceError;
use crate::schema::boards::dsl;
use actix_web::{put, web, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// ======================================================================
// DTOs

#[derive(Debug, Deserialize)]
pub struct UpdateBoardRequest {
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateBoardResponse {
    board: Board,
}

// ======================================================================
// Route

#[put("/{id}")]
pub async fn update_board(
    pool: web::Data<DbPool>,
    board_id: web::Path<i32>,
    data: web::Json<UpdateBoardRequest>,
) -> Result<HttpResponse, ServiceError> {
    let board_id = board_id.into_inner();
    let update_data = UpdateBoard {
        name: data.name.clone(),
    };

    let updated_board = update_existing_board(&pool, board_id, update_data)?;
    Ok(HttpResponse::Ok().json(UpdateBoardResponse {
        board: updated_board,
    }))
}

// ======================================================================
// Database operations

fn update_existing_board(
    pool: &DbPool,
    board_id: i32,
    update_data: UpdateBoard,
) -> Result<Board, ServiceError> {
    let mut conn = get_db_connection(pool)?;
    diesel::update(dsl::boards.find(board_id))
        .set(&update_data)
        .get_result(&mut conn)
        .map_err(From::from)
}
