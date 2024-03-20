use actix_web::web;

mod availability;
mod buildings;
mod dev;
mod reservations;
mod spaces;
mod users;
mod validation;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .configure(buildings::config)
            .configure(spaces::config)
            .configure(spaces::images::config)
            .configure(spaces::reviews::config)
            .configure(spaces::summaries::config)
            .configure(users::profiles::config)
            .configure(users::reviews::config)
            .configure(users::summaries::config)
            .configure(users::notifs::config)
            .configure(reservations::config)
            .configure(availability::config)
            .configure(dev::config),
    );
}
