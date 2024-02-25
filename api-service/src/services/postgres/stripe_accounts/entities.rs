use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

// ======================================================================
// DB Entity

#[derive(Debug, FromRow)]
pub struct StripeAccount {
    pub id: i32,
    pub user_id: Uuid,
    pub account_id: String,
    pub last_modified: NaiveDateTime,
    pub created_at: NaiveDateTime,
}
