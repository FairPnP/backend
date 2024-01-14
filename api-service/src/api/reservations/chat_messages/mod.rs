use actix_web::web;

mod create;
mod list;
pub mod public;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/chat")
            .service(create::create_chat_message)
            .service(list::list_chat_messages),
    );
}
