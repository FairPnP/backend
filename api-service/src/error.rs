use std::fmt;

use actix_web::{HttpResponse, ResponseError};

#[derive(Debug)]
pub enum ServiceError {
    DatabaseError(String),
    InternalError(String),
    Unauthorized,
    BadRequest(String),
    PoolError(String),
    RowNotFound,
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

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::DatabaseError(ref message) => {
                HttpResponse::InternalServerError().json(message)
            }
            ServiceError::InternalError(ref message) => {
                HttpResponse::InternalServerError().json(message)
            }
            ServiceError::Unauthorized => HttpResponse::Unauthorized().finish(),
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::RowNotFound => HttpResponse::NotFound().finish(),
            ServiceError::PoolError(ref message) => {
                HttpResponse::InternalServerError().json(message)
            }
        }
    }

    fn status_code(&self) -> reqwest::StatusCode {
        reqwest::StatusCode::INTERNAL_SERVER_ERROR
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
        }
    }
}
