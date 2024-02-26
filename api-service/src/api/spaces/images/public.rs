use crate::{services::postgres::spaces::images::entities::SpaceImage, utils::hashids::encode_id};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicSpaceImage {
    pub id: String,
    pub space_id: String,
    pub slot_id: i32,
    pub img_url: String,
}

impl From<SpaceImage> for PublicSpaceImage {
    fn from(space_image: SpaceImage) -> Self {
        PublicSpaceImage {
            id: encode_id(space_image.id),
            space_id: encode_id(space_image.space_id),
            slot_id: space_image.slot_id,
            img_url: space_image.img_url,
        }
    }
}
