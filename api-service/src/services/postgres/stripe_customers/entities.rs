use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

// ======================================================================
// DB Entity

#[derive(Debug, FromRow)]
pub struct StripeCustomer {
    pub id: i32,
    pub user_id: Uuid,
    pub customer_id: String,
    pub last_modified: NaiveDateTime,
    pub created_at: NaiveDateTime,
}
