use crate::board::entities::Board;
use crate::db::{get_db_connection, DbPool};
use crate::error::ServiceError;
use crate::schema::boards::dsl;
use crate::users::get_user_id;
use actix_web::{get, web, HttpRequest, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ======================================================================
// DTOs

#[derive(Debug, Serialize)]
pub struct ListBoardsResponse {
    pub boards: Vec<Board>,
    pub next_cursor: Option<i32>,
    pub current_limit: i64,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    after: Option<i32>,
    limit: Option<i64>,
}

// ======================================================================
// Route

#[get("")]
pub async fn list_boards(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;

    let limit = query.limit.unwrap_or(10);
    let boards = get_boards_by_team_id(&pool, user_id, query.after, limit)?;
    let next_cursor = if boards.len() as i64 == limit {
        boards.last().map(|b| b.id)
    } else {
        None
    };

    Ok(HttpResponse::Ok().json(ListBoardsResponse {
        boards,
        next_cursor,
        current_limit: limit,
    }))
}

// ======================================================================
// Database operations

fn get_boards_by_team_id(
    pool: &DbPool,
    team_id: Uuid,
    after: Option<i32>,
    limit: i64,
) -> Result<Vec<Board>, ServiceError> {
    let mut conn = get_db_connection(pool)?;
    let mut query = dsl::boards
        .filter(dsl::team_id.eq(team_id))
        .order(dsl::id.asc())
        .into_boxed();

    if let Some(after) = after {
        query = query.filter(dsl::id.gt(after));
    }

    query
        .limit(limit)
        .load::<Board>(&mut conn)
        .map_err(From::from)
}
