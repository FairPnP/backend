use actix_web::web;

mod complete;
mod delete;
mod list;
pub mod public;
mod read;
mod s3;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/space_images/v1")
            .service(read::read_space_image)
            .service(complete::complete_space_image)
            .service(delete::delete_space_image)
            .service(list::list_space_images)
            .service(s3::create_space_image),
    );
}
