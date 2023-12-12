use actix_web::web;

mod buildings;
mod dev;
mod spaces;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .configure(buildings::config)
            .configure(spaces::config)
            .configure(dev::config),
    );
}
