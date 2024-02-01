use actix_web::web;

pub mod public;
mod read;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/space_summaries/v1").service(read::read_space_summary));
}
