use actix_web::{get, web, HttpResponse};

use crate::{db::DbPool, error::ServiceError};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/health").service(health_check));
}

#[get("")]
pub async fn health_check(pool: web::Data<DbPool>) -> Result<HttpResponse, ServiceError> {
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(pool.get_ref())
        .await?;

    if row.0 != 150 {
        return Err(ServiceError::InternalError(
            "Health check failed".to_string(),
        ));
    }

    Ok(HttpResponse::Ok().finish())
}
