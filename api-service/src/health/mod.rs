use actix_web::{get, web, HttpResponse, Responder};
use diesel::connection::SimpleConnection;

use crate::db::DbPool;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/health").service(health_check));
}

#[get("")]
pub async fn health_check(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    match conn.batch_execute("SELECT 1") {
        Ok(_) => HttpResponse::Ok().body("Healthy"),
        Err(_) => HttpResponse::ServiceUnavailable().body("Unhealthy"),
    }
}
