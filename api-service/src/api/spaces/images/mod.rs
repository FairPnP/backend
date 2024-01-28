use actix_web::web;

mod create;
mod delete;
mod list;
pub mod public;
mod read;
mod s3;
mod update;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/images")
            .service(create::create_space_image)
            .service(read::read_space_image)
            .service(update::update_space_image)
            .service(delete::delete_space_image)
            .service(list::list_space_images)
            .service(s3::generate_presigned_url),
    );
}
