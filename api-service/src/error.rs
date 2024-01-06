use std::fmt;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use stripe::StripeError;
use validator::ValidationErrors;

#[derive(Serialize)]
struct ErrorResponse {
    error: ErrorDetails,
}

#[derive(Serialize)]
struct ErrorDetails {
    #[serde(rename = "type")]
    error_type: String,
    message: String,
}

#[derive(Debug)]
pub enum ServiceError {
    DatabaseError(String),
    InternalError(String),
    Unauthorized,
    BadRequest(String),
    PoolError(String),
    RowNotFound,
    ValidationError(ValidationErrors),
}

impl From<sqlx::Error> for ServiceError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => ServiceError::RowNotFound,
            sqlx::Error::Database(e) => ServiceError::DatabaseError(e.to_string()),
            sqlx::Error::PoolTimedOut => {
                ServiceError::PoolError("Connection pool timeout".to_string())
            }
            _ => ServiceError::DatabaseError(error.to_string()),
        }
    }
}

impl From<StripeError> for ServiceError {
    fn from(error: StripeError) -> Self {
        ServiceError::InternalError(error.to_string())
    }
}

impl From<ValidationErrors> for ServiceError {
    fn from(error: ValidationErrors) -> Self {
        ServiceError::ValidationError(error)
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        let (status_code, error_type, error_message) = match *self {
            ServiceError::DatabaseError(ref message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database Error",
                message.to_string(),
            ),
            ServiceError::InternalError(ref message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Error",
                message.to_string(),
            ),
            ServiceError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "Unauthorized",
                "Unauthorized".to_string(),
            ),
            ServiceError::BadRequest(ref message) => {
                (StatusCode::BAD_REQUEST, "Bad Request", message.to_string())
            }
            ServiceError::RowNotFound => {
                (StatusCode::NOT_FOUND, "Not Found", "Not Found".to_string())
            }
            ServiceError::PoolError(ref message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Pool Error",
                message.to_string(),
            ),
            ServiceError::ValidationError(ref errors) => (
                StatusCode::BAD_REQUEST,
                "Validation Error",
                errors.to_string(),
            ),
        };

        let error_response = ErrorResponse {
            error: ErrorDetails {
                error_type: error_type.to_string(),
                message: error_message,
            },
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ServiceError::DatabaseError(ref message) => write!(f, "Database Error: {}", message),
            ServiceError::InternalError(ref message) => write!(f, "Internal Error: {}", message),
            ServiceError::Unauthorized => write!(f, "Unauthorized"),
            ServiceError::BadRequest(ref message) => write!(f, "Bad Request: {}", message),
            ServiceError::PoolError(ref message) => write!(f, "Pool Error: {}", message),
            ServiceError::RowNotFound => write!(f, "Row not found"),
            ServiceError::ValidationError(ref errors) => {
                write!(f, "Validation Error: {}", errors)
            }
        }
    }
}
