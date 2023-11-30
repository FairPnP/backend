use actix_web::web;

mod buildings;
mod dev;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .configure(buildings::routes::config)
            .configure(dev::config),
    );
}
