use actix_web::web;

mod create;
mod read;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/customers")
            .service(read::read_customer)
            .service(create::create_customer),
    );
}
