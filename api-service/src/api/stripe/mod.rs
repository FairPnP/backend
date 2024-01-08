use actix_web::web;

mod accounts;
mod customers;
mod payments;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("stripe/v1")
            .configure(accounts::config)
            .configure(customers::config)
            .configure(payments::config),
    );
}
