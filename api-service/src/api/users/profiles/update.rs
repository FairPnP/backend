use crate::{
    api::validation::validate_req_data,
    auth::user::get_user_id,
    error::ServiceError,
    services::{
        postgres::{users::profiles::UserProfileDb, DbPool},
        s3::presigned::is_valid_url,
    },
};
use actix_web::{put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicUserProfile;
use super::S3_BUCKET_AVATAR_PATH;

// ======================================================================
// DTOs

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserProfileRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(min = 1))]
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateUserProfileResponse {
    user_profile: PublicUserProfile,
}

// ======================================================================
// Route

#[put("")]
pub async fn update_user_profile(
    pool: web::Data<DbPool>,
    req: actix_web::HttpRequest,
    data: web::Json<UpdateUserProfileRequest>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let data = validate_req_data(data.into_inner())?;

    // if avatar_url is provided, validate it
    if let Some(avatar_url) = &data.avatar_url {
        if !is_valid_url(avatar_url, user_id, S3_BUCKET_AVATAR_PATH) {
            return Err(ServiceError::BadRequest("Invalid avatar_url".to_string()));
        }
    }

    // check if user profile exists
    let user_profile = UserProfileDb::get(&pool, user_id)
        .await
        .map_err(|err| ServiceError::from(err));

    // create or update user profile
    let updated_user_profile = match user_profile {
        Ok(_) => UserProfileDb::update(&pool, user_id, data.name, data.avatar_url).await?,
        Err(ServiceError::NotFound) => {
            let name = match data.name {
                Some(name) => name,
                None => return Err(ServiceError::BadRequest("name is required".to_string())),
            };
            UserProfileDb::insert(&pool, user_id, name, data.avatar_url).await?
        }
        Err(err) => return Err(err),
    };

    Ok(HttpResponse::Ok().json(UpdateUserProfileResponse {
        user_profile: updated_user_profile.into(),
    }))
}
