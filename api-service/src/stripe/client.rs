use reqwest::{Client, Method};
use serde::de::DeserializeOwned;
use std::fmt::Debug;

use super::error::{ApiError, InternalError, StripeError};

#[derive(Debug, Clone)]
pub struct StripeClient {
    secret_key: String,
    pub return_url: String,
    pub refresh_url: String,
    http_client: Client,
}

impl StripeClient {
    pub fn new(secret_key: String, return_url: String, refresh_url: String) -> StripeClient {
        StripeClient {
            secret_key,
            return_url,
            refresh_url,
            http_client: Client::new(),
        }
    }

    pub async fn request<T: DeserializeOwned + Debug>(
        &self,
        method: Method,
        path: &str,
        params: Option<Vec<(&str, &str)>>,
    ) -> Result<T, StripeError> {
        let url = format!("https://api.stripe.com/v1{}", path);
        let mut request = self
            .http_client
            .request(method, &url)
            .header("Authorization", format!("Bearer {}", self.secret_key))
            .header("Content-Type", "application/x-www-form-urlencoded");

        if let Some(p) = params {
            request = request.form(&p);
        }

        let res = request.send().await;

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    let parsed = response.json::<T>().await;
                    match parsed {
                        Ok(data) => Ok(data),
                        Err(err) => {
                            // Log or handle deserialization error
                            Err(StripeError::InternalError(InternalError {
                                message: format!("Failed to deserialize response: {:?}", err),
                            }))
                        }
                    }
                } else {
                    // Handle API errors
                    let error_response = response.json::<ApiError>().await.unwrap_or(ApiError {
                        error_type: "Unknown".to_string(),
                        message: "Unknown error occurred".to_string(),
                    });
                    Err(StripeError::ApiError(error_response))
                }
            }
            Err(err) => {
                // Handle network or request errors
                Err(StripeError::InternalError(InternalError {
                    message: format!("Request failed: {:?}", err),
                }))
            }
        }
    }
}
