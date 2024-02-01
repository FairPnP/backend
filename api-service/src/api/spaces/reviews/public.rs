use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::spaces::reviews::entities::SpaceReview;

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicSpaceReview {
    pub id: i32,
    pub user_id: Uuid,
    pub space_id: i32,
    pub message: String,
    pub stars: i32,
    pub created_at: chrono::NaiveDateTime,
    pub last_modified: chrono::NaiveDateTime,
}

impl From<SpaceReview> for PublicSpaceReview {
    fn from(space_image: SpaceReview) -> Self {
        PublicSpaceReview {
            id: space_image.id,
            user_id: space_image.user_id,
            space_id: space_image.space_id,
            message: space_image.message,
            stars: space_image.stars,
            created_at: space_image.created_at,
            last_modified: space_image.last_modified,
        }
    }
}
