use crate::{
    auth::user::get_user_id,
    error::ServiceError,
    services::postgres::{
        spaces::{images::SpaceImageDb, SpaceDb},
        DbPool,
    },
    utils::hashids::decode_id,
};
use actix_web::{delete, web, HttpResponse};

// ======================================================================
// Route

#[delete("/{id}")]
pub async fn delete_space_image(
    pool: web::Data<DbPool>,
    req: actix_web::HttpRequest,
    space_image_id: web::Path<String>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let space_image_id = decode_id(&space_image_id.into_inner())?;

    let space_image = SpaceImageDb::get(&pool, space_image_id).await?;

    let space = SpaceDb::get(&pool, space_image.space_id).await?;
    // if user is not the owner of the space, return unauthorized
    if space.user_id != user_id {
        return Err(ServiceError::Unauthorized);
    }

    SpaceImageDb::delete(&pool, space_image_id).await?;
    Ok(HttpResponse::Ok().finish())
}
