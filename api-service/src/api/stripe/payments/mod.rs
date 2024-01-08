use actix_web::web;

mod create;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/payments").service(create::create_payment));
}
