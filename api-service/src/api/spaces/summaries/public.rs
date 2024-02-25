use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::services::postgres::spaces::summaries::entities::SpaceSummary;

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicSpaceSummary {
    pub host_user_id: Uuid,
    pub space_id: i32,
    pub total_reviews: i32,
    pub average_stars: f64,
}

impl From<SpaceSummary> for PublicSpaceSummary {
    fn from(space_summary: SpaceSummary) -> Self {
        let fixed_average_stars = space_summary.average_stars as f64 / 100.0;
        PublicSpaceSummary {
            host_user_id: space_summary.host_user_id,
            space_id: space_summary.space_id,
            total_reviews: space_summary.total_reviews,
            average_stars: fixed_average_stars,
        }
    }
}
