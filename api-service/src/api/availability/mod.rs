use actix_web::web;

mod create;
mod delete;
mod list;
mod public;
mod read;
mod update;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/availability/v1")
            .service(create::create_availability)
            .service(read::read_availability)
            .service(update::update_availability)
            .service(delete::delete_availability)
            .service(list::list_availability),
    );
}
