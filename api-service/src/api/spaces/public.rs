use crate::{services::postgres::spaces::entities::Space, utils::hashids::encode_id};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct PublicSpace {
    pub id: String,
    pub user_id: Uuid,
    pub building_id: String,
    pub name: String,
    pub description: Option<String>,
    pub max_vehicle_size: String,
    pub coverage: String,
    pub height_clearance_cm: Option<i32>,
    pub access_restrictions: Option<String>,
    pub parking_instructions: Option<String>,
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
            max_vehicle_size: space.max_vehicle_size,
            coverage: space.coverage,
            height_clearance_cm: space.height_clearance_cm,
            access_restrictions: space.access_restrictions,
            parking_instructions: space.parking_instructions,
            img_urls: vec![],
        }
    }
}
