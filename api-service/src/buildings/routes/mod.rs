use actix_web::web;

mod create;
mod delete;
mod list;
mod read;
mod update;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/boards")
            .service(create::create_board)
            .service(read::read_board)
            .service(update::update_board)
            .service(delete::delete_board)
            .service(list::list_boards),
    );
}
