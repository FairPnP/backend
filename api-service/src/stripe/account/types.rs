use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountId(String);

impl AccountId {
    pub fn new(id: String) -> Self {
        AccountId(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub type Timestamp = i64;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountRequirements {
    pub currently_due: Option<Vec<String>>,
}

// =============================================================================
// Account Link

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountLink {
    pub created: Timestamp,

    pub expires_at: Timestamp,

    pub url: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct CreateAccountLink {
    pub account: AccountId,
    pub refresh_url: String,
    pub return_url: String,

    #[serde(rename = "type")]
    pub type_: String,
}

// =============================================================================
// Login Link

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoginLink {
    pub created: Timestamp,

    pub url: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct CreateLoginLink {
    pub account: AccountId,
}
