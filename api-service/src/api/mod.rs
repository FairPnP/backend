use actix_web::web;

mod availability;
mod buildings;
mod dev;
mod reservations;
mod spaces;
mod stripe;
mod validation;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .configure(buildings::config)
            .configure(spaces::config)
            .configure(reservations::config)
            .configure(availability::config)
            .configure(stripe::config)
            .configure(dev::config),
    );
}
