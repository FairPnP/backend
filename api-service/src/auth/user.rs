use actix_web::HttpRequest;
use uuid::Uuid;

use crate::error::ServiceError;

pub fn get_user_id(req: &HttpRequest) -> Result<Uuid, ServiceError> {
    match req.headers().get("X-Auth-User") {
        Some(user_id) => {
            let user_id = match user_id.to_str() {
                Ok(user_id) => user_id,
                Err(_) => return Err(ServiceError::Unauthorized),
            };
            let user_id = match Uuid::parse_str(user_id) {
                Ok(user_id) => user_id,
                Err(_) => {
                    return Err(ServiceError::Unauthorized);
                }
            };

            Ok(user_id)
        }
        None => Err(ServiceError::Unauthorized),
    }
}
