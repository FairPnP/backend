use validator::Validate;

use crate::error::ServiceError;

pub mod list_param;

pub fn validate_req_data<T: Validate>(data: T) -> Result<T, ServiceError> {
    match data.validate() {
        Ok(_) => Ok(data),
        Err(err) => Err(err.into()),
    }
}
