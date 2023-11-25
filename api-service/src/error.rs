use std::fmt;

use actix_web::{HttpResponse, ResponseError};

#[derive(Debug)]
pub enum ServiceError {
    NotFound(String),
    UniqueViolation(String),
    DatabaseError(String),
    InternalError(String),
    Unauthorized,
    BadRequest(String),
}

impl From<diesel::result::Error> for ServiceError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => {
                ServiceError::NotFound("Resource not found".to_string())
            }
            diesel::result::Error::DatabaseError(kind, _) => match kind {
                diesel::result::DatabaseErrorKind::UniqueViolation => {
                    ServiceError::UniqueViolation("Resource already exists".to_string())
                }
                _ => ServiceError::DatabaseError("Database operation failed".to_string()),
            },
            _ => ServiceError::InternalError("An internal error occurred".to_string()),
        }
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::NotFound(ref message) => HttpResponse::NotFound().json(message),
            ServiceError::UniqueViolation(ref message) => HttpResponse::Conflict().json(message),
            ServiceError::DatabaseError(ref message) => {
                HttpResponse::InternalServerError().json(message)
            }
            ServiceError::InternalError(ref message) => {
                HttpResponse::InternalServerError().json(message)
            }
            ServiceError::Unauthorized => HttpResponse::Unauthorized().finish(),
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
        }
    }

    fn status_code(&self) -> reqwest::StatusCode {
        reqwest::StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ServiceError::NotFound(ref message) => write!(f, "Not Found: {}", message),
            ServiceError::UniqueViolation(ref message) => {
                write!(f, "Unique Violation: {}", message)
            }
            ServiceError::DatabaseError(ref message) => write!(f, "Database Error: {}", message),
            ServiceError::InternalError(ref message) => write!(f, "Internal Error: {}", message),
            ServiceError::Unauthorized => write!(f, "Unauthorized"),
            ServiceError::BadRequest(ref message) => write!(f, "Bad Request: {}", message),
        }
    }
}
