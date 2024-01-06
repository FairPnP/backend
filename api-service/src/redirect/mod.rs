use actix_web::web;

mod stripe;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/redirect")
            .service(stripe::stripe_return)
            .service(stripe::stripe_refresh),
    );
}
