use serde::{Deserialize, Serialize};

use crate::services::postgres::users::notif_tokens::entities::UserNotifToken;

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicUserNotifToken {
    pub expo_token: Option<String>,
    pub device_token: String,
    pub device_type: String,
}

impl From<UserNotifToken> for PublicUserNotifToken {
    fn from(token: UserNotifToken) -> Self {
        PublicUserNotifToken {
            expo_token: token.expo_token,
            device_token: token.device_token,
            device_type: token.device_type,
        }
    }
}
