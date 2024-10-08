use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    services::postgres::spaces::reviews::entities::SpaceReview, utils::hashids::encode_id,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicSpaceReview {
    pub id: String,
    pub user_id: Uuid,
    pub space_id: String,
    pub message: String,
    pub stars: i32,
    pub created_at: chrono::NaiveDateTime,
    pub last_modified: chrono::NaiveDateTime,
}

impl From<SpaceReview> for PublicSpaceReview {
    fn from(space_image: SpaceReview) -> Self {
        PublicSpaceReview {
            id: encode_id(space_image.id),
            user_id: space_image.user_id,
            space_id: encode_id(space_image.space_id),
            message: space_image.message,
            stars: space_image.stars,
            created_at: space_image.created_at,
            last_modified: space_image.last_modified,
        }
    }
}
