use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::users::summaries::entities::UserSummary;

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicUserSummary {
    pub user_id: Uuid,
    pub total_reviews: i32,
    pub average_stars: f64,
}

impl From<UserSummary> for PublicUserSummary {
    fn from(user_summary: UserSummary) -> Self {
        let fixed_average_stars = user_summary.average_stars as f64 / 100.0;
        PublicUserSummary {
            user_id: user_summary.user_id,
            total_reviews: user_summary.total_reviews,
            average_stars: fixed_average_stars,
        }
    }
}
