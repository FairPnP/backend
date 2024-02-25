use serde::{Deserialize, Serialize};

use crate::services::stripe::account::types::AccountId;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CustomerId(String);

impl CustomerId {
    pub fn new(id: String) -> Self {
        CustomerId(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EphemeralKeyId(String);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EphemeralKey {
    pub id: EphemeralKeyId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

// =============================================================================
// Payment Intent

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentIntent {
    pub id: String,
    pub amount: i64,
    pub client_secret: String,
    pub customer: CustomerId,
}

#[derive(Clone, Debug, Serialize)]
pub struct CreatePaymentIntent {
    pub amount: i64,
    pub currency: String,
    pub customer: CustomerId,
    pub application_fee_amount: Option<i64>,
    #[serde(rename = "transfer_data[destination]")]
    pub transfer_destination: Option<AccountId>,
}
