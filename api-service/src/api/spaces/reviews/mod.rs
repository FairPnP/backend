use actix_web::web;

mod create;
mod delete;
mod list;
pub mod public;
mod read;
mod update;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/space_reviews/v1")
            .service(create::create_space_review)
            .service(read::read_space_review)
            .service(delete::delete_space_review)
            .service(list::list_space_reviews)
            .service(update::update_space_review),
    );
}
