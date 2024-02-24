use actix_web::web;

pub mod public;
mod token;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user_notifs/v1")
            .service(token::update_token)
            .service(token::list_tokens),
    );
}
