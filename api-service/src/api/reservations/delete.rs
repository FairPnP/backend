use crate::{
    auth::user::get_user_id,
    error::ServiceError,
    services::postgres::{reservations::ReservationDb, DbPool},
    utils::hashids::decode_id,
};
use actix_web::{delete, web, HttpResponse};

// ======================================================================
// Route

#[delete("/{id}")]
pub async fn delete_reservation(
    pool: web::Data<DbPool>,
    req: actix_web::HttpRequest,
    reservation_id: web::Path<String>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let reservation_id = decode_id(&reservation_id.into_inner())?;

    ReservationDb::delete(&pool, user_id, reservation_id).await?;
    Ok(HttpResponse::Ok().finish())
}
