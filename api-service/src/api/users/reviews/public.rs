use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::services::postgres::users::reviews::entities::UserReview;

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicUserReview {
    pub id: i32,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub message: String,
    pub stars: i32,
    pub created_at: chrono::NaiveDateTime,
    pub last_modified: chrono::NaiveDateTime,
}

impl From<UserReview> for PublicUserReview {
    fn from(user_image: UserReview) -> Self {
        PublicUserReview {
            id: user_image.id,
            from_user_id: user_image.from_user_id,
            to_user_id: user_image.to_user_id,
            message: user_image.message,
            stars: user_image.stars,
            created_at: user_image.created_at,
            last_modified: user_image.last_modified,
        }
    }
}
