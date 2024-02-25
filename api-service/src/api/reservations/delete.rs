use crate::{
    auth::user::get_user_id,
    services::postgres::{reservations::ReservationDb, DbPool},
    error::ServiceError,
};
use actix_web::{delete, web, HttpResponse};

// ======================================================================
// Route

#[delete("/{id}")]
pub async fn delete_reservation(
    pool: web::Data<DbPool>,
    req: actix_web::HttpRequest,
    reservation_id: web::Path<i32>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;

    ReservationDb::delete(&pool, user_id, reservation_id.into_inner()).await?;
    Ok(HttpResponse::Ok().finish())
}
