use crate::db::spaces::entities::Space;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PublicSpace {
    pub id: i32,
    pub building_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub picture_url: Option<String>,
    pub max_vehicle_size: String,
    pub coverage: String,
    pub height_clearance_cm: Option<i32>,
    pub access_restrictions: Option<String>,
    pub parking_instructions: Option<String>,
}

impl From<Space> for PublicSpace {
    fn from(space: Space) -> Self {
        PublicSpace {
            id: space.id,
            building_id: space.building_id,
            name: space.name,
            description: space.description,
            picture_url: space.picture_url,
            max_vehicle_size: space.max_vehicle_size,
            coverage: space.coverage,
            height_clearance_cm: space.height_clearance_cm,
            access_restrictions: space.access_restrictions,
            parking_instructions: space.parking_instructions,
        }
    }
}
