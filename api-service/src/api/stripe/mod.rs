use actix_web::web;

mod account;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("stripe/v1").configure(account::config));
}
