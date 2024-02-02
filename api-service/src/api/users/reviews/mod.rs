use actix_web::web;

mod create;
mod delete;
mod list;
pub mod public;
mod read;
mod update;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user_reviews/v1")
            .service(create::create_user_review)
            .service(read::read_user_review)
            .service(delete::delete_user_review)
            .service(list::list_user_reviews)
            .service(update::update_user_review),
    );
}
