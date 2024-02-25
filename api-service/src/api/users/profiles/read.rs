use crate::{
    services::postgres::{users::profiles::UserProfileDb, DbPool},
    error::ServiceError,
};
use actix_web::{get, web, HttpResponse};
use serde::Serialize;

use super::public::PublicUserProfile;

// ======================================================================
// DTOs

#[derive(Debug, Serialize)]
pub struct ReadUserProfileResponse {
    user_profile: PublicUserProfile,
}

// ======================================================================
// Route

#[get("{user_id}")]
pub async fn read_user_profile(
    pool: web::Data<DbPool>,
    user_id: web::Path<String>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = user_id
        .parse()
        .map_err(|_| ServiceError::BadRequest("Invalid user_id".to_string()))?;

    let user_profile = UserProfileDb::get(&pool, user_id).await?;
    Ok(HttpResponse::Ok().json(ReadUserProfileResponse {
        user_profile: user_profile.into(),
    }))
}
