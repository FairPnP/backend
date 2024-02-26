use crate::{
    auth::user::get_user_id,
    error::ServiceError,
    services::postgres::{availability::AvailabilityDb, DbPool},
    utils::hashids::decode_id,
};
use actix_web::{delete, web, HttpResponse};

// ======================================================================
// Route

#[delete("/{id}")]
pub async fn delete_availability(
    pool: web::Data<DbPool>,
    req: actix_web::HttpRequest,
    availability_id: web::Path<String>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let id = decode_id(&availability_id.into_inner())?;

    AvailabilityDb::delete(&pool, user_id, id).await?;
    Ok(HttpResponse::Ok().finish())
}
