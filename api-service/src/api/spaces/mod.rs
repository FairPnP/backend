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
        web::scope("/spaces/v1")
            .service(create::create_space)
            .service(read::read_space)
            .service(update::update_space)
            .service(delete::delete_space)
            .service(list::list_spaces)
            .service(s3::generate_presigned_url),
    );
}
