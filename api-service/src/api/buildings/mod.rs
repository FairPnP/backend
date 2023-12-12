use actix_web::web;

mod create;
mod delete;
mod list;
mod public;
mod read;
mod update;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/buildings/v1")
            .service(create::create_building)
            .service(read::read_building)
            .service(update::update_building)
            .service(delete::delete_building)
            .service(list::list_buildings),
    );
}
