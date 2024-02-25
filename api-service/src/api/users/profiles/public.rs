use crate::services::postgres::users::profiles::entities::UserProfile;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicUserProfile {
    pub user_id: Uuid,
    pub name: String,
    pub avatar_url: Option<String>,
}

impl From<UserProfile> for PublicUserProfile {
    fn from(user_profile: UserProfile) -> Self {
        PublicUserProfile {
            user_id: user_profile.user_id,
            name: user_profile.name,
            avatar_url: user_profile.avatar_url,
        }
    }
}
