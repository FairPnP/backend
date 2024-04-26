use crate::{services::postgres::spaces::entities::Space, utils::hashids::encode_id};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct PublicSpace {
    pub id: String,
    pub user_id: Uuid,
    pub building_id: String,
    pub name: String,
    pub description: String,
    pub img_urls: Vec<String>,
}

impl From<Space> for PublicSpace {
    fn from(space: Space) -> Self {
        PublicSpace {
            id: encode_id(space.id),
            user_id: space.user_id,
            building_id: encode_id(space.building_id),
            name: space.name,
            description: space.description,
            img_urls: vec![],
        }
    }
}
