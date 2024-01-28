use crate::db::space_images::entities::SpaceImage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicSpaceImage {
    pub id: i32,
    pub space_id: i32,
    pub slot_id: i32,
    pub img_url: String,
}

impl From<SpaceImage> for PublicSpaceImage {
    fn from(space_image: SpaceImage) -> Self {
        PublicSpaceImage {
            id: space_image.id,
            space_id: space_image.space_id,
            slot_id: space_image.slot_id,
            img_url: space_image.img_url,
        }
    }
}
