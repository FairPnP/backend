use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum StripeError {
    ApiError(ApiError),
    InternalError(InternalError),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiError {
    pub error_type: String,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InternalError {
    pub message: String,
}
