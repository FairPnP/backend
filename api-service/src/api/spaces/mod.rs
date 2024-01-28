use actix_web::web;

mod create;
mod delete;
mod images;
mod list;
pub mod public;
mod read;
mod update;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/spaces/v1")
            .configure(images::config)
            .service(create::create_space)
            .service(read::read_space)
            .service(update::update_space)
            .service(delete::delete_space)
            .service(list::list_spaces),
    );
}
