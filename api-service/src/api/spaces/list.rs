use crate::{
    api::validation::validate_req_data,
    auth::user::get_user_id,
    error::ServiceError,
    services::postgres::{
        spaces::{images::SpaceImageDb, SpaceDb},
        DbPool,
    },
};
use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::public::PublicSpace;

// ======================================================================
// DTOs

#[derive(Deserialize, Validate)]
pub struct PaginationParams {
    #[validate(range(min = 1))]
    offset_id: Option<i32>,
    #[validate(range(min = 1))]
    limit: Option<i32>,
    user: Option<bool>,
    #[validate(length(min = 16, max = 32))]
    building_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListSpacesResponse {
    pub spaces: Vec<PublicSpace>,
    pub next_offset_id: Option<i32>,
    pub limit: i32,
}

// ======================================================================
// Route

#[get("")]
pub async fn list_spaces(
    req: actix_web::HttpRequest,
    pool: web::Data<DbPool>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = get_user_id(&req)?;
    let query = validate_req_data(query.into_inner())?;
    // default to user_id, but allow override
    let user = match query.user {
        // allow override
        Some(val) => match val {
            // if true, use user_id
            true => Some(user_id.clone()),
            false => None,
        },
        // default to user_id
        None => Some(user_id.clone()),
    };

    // limit default to 10, max 20
    let limit = query.limit.map_or(10, |l| if l > 20 { 20 } else { l });
    let spaces = SpaceDb::list(
        &pool,
        query.offset_id,
        limit,
        user,
        query.building_id.to_owned(),
    )
    .await?;
    let next_offset_id = if spaces.len() as i32 == limit {
        spaces.last().map(|b| b.id)
    } else {
        None
    };

    let ids = spaces.iter().map(|s| s.id).collect::<Vec<i32>>();
    // get the space images
    let img_map = SpaceImageDb::list_for_spaces(&pool, &ids).await?;
    // populate the img_urls field
    let spaces = spaces
        .into_iter()
        .map(|s| {
            let id = s.id;
            let mut pub_s = PublicSpace::from(s);
            pub_s.img_urls = img_map
                .get(&id)
                .map(|imgs| imgs.iter().map(|img| img.img_url.to_owned()).collect())
                .unwrap_or_default();
            pub_s
        })
        .collect::<Vec<PublicSpace>>();

    Ok(HttpResponse::Ok().json(ListSpacesResponse {
        spaces,
        next_offset_id,
        limit,
    }))
}
