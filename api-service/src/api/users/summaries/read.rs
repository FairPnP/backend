use crate::{
    db::{users::summaries::UserSummaryDb, DbPool},
    error::ServiceError,
};
use actix_web::{get, web, HttpResponse};
use serde::Serialize;
use uuid::Uuid;

use super::public::PublicUserSummary;

// ======================================================================
// DTOs

#[derive(Debug, Serialize)]
pub struct ReadUserSummaryResponse {
    user_summary: PublicUserSummary,
}

// ======================================================================
// Route

#[get("/{id}")]
pub async fn read_user_summary(
    pool: web::Data<DbPool>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    let user_summary = UserSummaryDb::get(&pool, *user_id).await?;
    Ok(HttpResponse::Ok().json(ReadUserSummaryResponse {
        user_summary: user_summary.into(),
    }))
}
