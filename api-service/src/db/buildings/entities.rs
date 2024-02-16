use chrono::NaiveDateTime;
use sqlx::types::BigDecimal;
use sqlx::FromRow;

// ======================================================================
// DB Entity

#[derive(Debug, FromRow)]
pub struct Building {
    pub id: i32,
    pub name: String,
    pub place_id: String,
    pub latitude: BigDecimal,
    pub longitude: BigDecimal,
    pub street_number: String,
    pub street_name: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
    pub last_modified: NaiveDateTime,
    pub created_at: NaiveDateTime,
}
