use actix_web::web;

mod list;
pub mod public;
mod read;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/buildings/v1")
            .service(read::read_building)
            .service(list::list_buildings),
    );
}
