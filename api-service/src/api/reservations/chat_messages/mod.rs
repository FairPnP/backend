use actix_web::web;

mod create;
mod list_conversations;
mod list_messages;
pub mod public;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/chat")
            .service(create::create_chat_message)
            .service(list_conversations::list_host_conversations)
            .service(list_conversations::list_guest_conversations)
            .service(list_messages::list_chat_messages),
    );
}
