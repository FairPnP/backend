use actix_web::web;

mod create;
mod read;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/account")
            .service(create::create_account)
            .service(read::read_account),
    );
}
