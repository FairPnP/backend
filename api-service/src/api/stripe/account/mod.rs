use actix_web::web;

mod create;
mod dashboard;
mod read;
mod validate;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/account")
            .service(dashboard::dashboard)
            .service(validate::validate_account)
            .service(create::create_account)
            .service(read::read_account),
    );
}
