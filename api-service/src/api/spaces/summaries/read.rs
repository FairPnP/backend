use crate::{
    error::ServiceError,
    services::postgres::{spaces::summaries::SpaceSummaryDb, DbPool},
    utils::hashids::decode_id,
};
use actix_web::{get, web, HttpResponse};
use serde::Serialize;

use super::public::PublicSpaceSummary;

// ======================================================================
// DTOs

#[derive(Debug, Serialize)]
pub struct ReadSpaceSummaryResponse {
    space_summary: PublicSpaceSummary,
}

// ======================================================================
// Route

#[get("/{id}")]
pub async fn read_space_summary(
    pool: web::Data<DbPool>,
    space_summary_id: web::Path<String>,
) -> Result<HttpResponse, ServiceError> {
    let space_summary_id = decode_id(&space_summary_id.into_inner())?;
    let space_summary = SpaceSummaryDb::get(&pool, space_summary_id).await?;

    Ok(HttpResponse::Ok().json(ReadSpaceSummaryResponse {
        space_summary: space_summary.into(),
    }))
}
