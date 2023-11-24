use crate::board::entities::{Board, NewBoard};
use crate::db::{get_db_connection, DbPool};
use crate::error::ServiceError;
use crate::schema::boards::dsl;
use crate::users::get_user_id;
use actix_web::{post, web, HttpRequest, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// ======================================================================
// DTOs

#[derive(Debug, Deserialize)]
pub struct CreateBoardRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateBoardResponse {
    pub board: Board,
}

// ======================================================================
// Route

#[post("")]
pub async fn create_board(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    data: web::Json<CreateBoardRequest>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let new_board = NewBoard {
        team_id: user_id,
        name: data.name.clone(),
    };

    let board = insert_new_board(&pool, new_board)?;
    Ok(HttpResponse::Created().json(CreateBoardResponse { board }))
}

// ======================================================================
// Database operations

fn insert_new_board(pool: &DbPool, new_board: NewBoard) -> Result<Board, ServiceError> {
    let mut conn = get_db_connection(pool)?;
    diesel::insert_into(dsl::boards)
        .values(&new_board)
        .get_result(&mut conn)
        .map_err(From::from)
}
