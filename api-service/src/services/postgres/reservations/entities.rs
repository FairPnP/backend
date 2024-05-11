use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt;
use uuid::Uuid;

// ======================================================================
// DB Entity

#[derive(Debug, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "reservation_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ReservationStatus {
    Pending,
    Confirmed,
    Failed,
    Cancelled,
    Timeout,
}

impl fmt::Display for ReservationStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status_str = match self {
            ReservationStatus::Pending => "pending",
            ReservationStatus::Confirmed => "confirmed",
            ReservationStatus::Failed => "failed",
            ReservationStatus::Cancelled => "cancelled",
            ReservationStatus::Timeout => "timeout",
        };
        write!(f, "{}", status_str)
    }
}

impl ReservationStatus {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "pending" => Ok(ReservationStatus::Pending),
            "confirmed" => Ok(ReservationStatus::Confirmed),
            "failed" => Ok(ReservationStatus::Failed),
            "cancelled" => Ok(ReservationStatus::Cancelled),
            "timeout" => Ok(ReservationStatus::Timeout),
            _ => Err(format!("Invalid reservation status: {}", s)),
        }
    }
}

#[derive(Debug, FromRow)]
pub struct Reservation {
    pub id: i32,
    pub user_id: Uuid,
    pub space_id: i32,
    pub availability_id: i32,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub status: ReservationStatus,
    pub last_modified: NaiveDateTime,
    pub created_at: NaiveDateTime,
}
