use actix_web::web;

mod reset_db;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/dev").service(reset_db::reset_database));
}
