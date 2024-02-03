use actix_web::web;

mod presigned;
pub mod public;
mod read;
mod update;

pub const S3_BUCKET_AVATAR_PATH: &str = "profile/avatars";

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user_profiles/v1")
            .service(presigned::create_avatar_presigned_url)
            .service(read::read_user_profile)
            .service(update::update_user_profile),
    );
}
