use actix_web::web;

mod chat_messages;
mod create;
mod delete;
mod list;
mod list_for_host;
pub mod public;
mod read;
mod update;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/reservations/v1")
            .configure(chat_messages::config)
            .service(list_for_host::list_host_reservations)
            .service(create::create_reservation)
            .service(read::read_reservation)
            .service(update::update_reservation)
            .service(delete::delete_reservation)
            .service(list::list_reservations),
    );
}
