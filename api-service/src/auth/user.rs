use actix_web::{HttpMessage, HttpRequest};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::ServiceError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub client_id: String,
    pub username: String,
}

// pub fn get_user(req: &HttpRequest) -> Result<User, ServiceError> {
//     match req.extensions().get::<User>() {
//         Some(user) => Ok(user.clone()),
//         None => Err(ServiceError::Unauthorized),
//     }
// }

pub fn get_user_id(req: &HttpRequest) -> Result<Uuid, ServiceError> {
    match req.extensions().get::<User>() {
        Some(user) => Ok(user.id),
        None => Err(ServiceError::Unauthorized),
    }
}
