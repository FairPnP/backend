use actix_web::web;

mod availability;
mod buildings;
mod dev;
mod spaces;
mod validation;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .configure(buildings::config)
            .configure(spaces::config)
            .configure(availability::config)
            .configure(dev::config),
    );
}
