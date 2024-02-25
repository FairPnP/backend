use crate::{
    auth::user::get_user_id,
    services::postgres::{availability::AvailabilityDb, DbPool},
    error::ServiceError,
};
use actix_web::{delete, web, HttpResponse};

// ======================================================================
// Route

#[delete("/{id}")]
pub async fn delete_availability(
    pool: web::Data<DbPool>,
    req: actix_web::HttpRequest,
    availability_id: web::Path<i32>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;

    AvailabilityDb::delete(&pool, user_id, availability_id.into_inner()).await?;
    Ok(HttpResponse::Ok().finish())
}
