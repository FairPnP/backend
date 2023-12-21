use actix_web::web;

mod create;
mod delete;
mod list;
pub mod public;
mod read;
mod update;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/reservations/v1")
            .service(create::create_reservation)
            .service(read::read_reservation)
            .service(update::update_reservation)
            .service(delete::delete_reservation)
            .service(list::list_reservations),
    );
}
