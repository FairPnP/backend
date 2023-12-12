use serde::Serialize;

use crate::db::spaces::entities::Space;

#[derive(Debug, Serialize)]
pub struct PublicSpace {
    pub id: i32,
    pub building_id: i32,
    pub name: String,
}

impl From<Space> for PublicSpace {
    fn from(space: Space) -> Self {
        PublicSpace {
            id: space.id,
            building_id: space.building_id,
            name: space.name,
        }
    }
}
