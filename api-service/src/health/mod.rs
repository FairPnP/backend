use actix_web::{get, web, HttpResponse};
use futures::try_join;
use rusoto_s3::S3Client;

use crate::{
    db::{self, s3, DbPool},
    error::ServiceError,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/health").service(health_check));
}

#[get("")]
pub async fn health_check(
    pool: web::Data<DbPool>,
    s3_client: web::Data<S3Client>,
) -> Result<HttpResponse, ServiceError> {
    let db_check = db::do_health_check(&pool);
    let s3_check = s3::do_health_check(&s3_client);

    try_join!(db_check, s3_check)?;

    Ok(HttpResponse::Ok().finish())
}
