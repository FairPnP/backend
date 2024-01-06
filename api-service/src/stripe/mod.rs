use reqwest::{Client, Error, Method, Response};

use crate::error::ServiceError;

#[derive(Debug, Clone)]
pub struct StripeClient {
    secret_key: String,
    return_url: String,
    refresh_url: String,
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

    pub async fn request(
        &self,
        method: Method,
        path: &str,
        params: Option<Vec<(&str, &str)>>,
    ) -> Result<Response, Error> {
        let url = format!("https://api.stripe.com/v1{}", path);
        let mut request = self
            .http_client
            .request(method, &url)
            .header("Authorization", format!("Bearer {}", self.secret_key))
            .header("Content-Type", "application/x-www-form-urlencoded");

        if let Some(p) = params {
            request = request.form(&p);
        }

        request.send().await
    }

    async fn parse_response(response: Response) -> Result<serde_json::Value, ServiceError> {
        let res = response;
        if res.status().is_success() {
            let json = res
                .json::<serde_json::Value>()
                .await
                .map_err(|_| ServiceError::InternalError("Failed to parse response".to_string()))?;
            Ok(json)
        } else {
            Err(ServiceError::InternalError("Request failed".to_string()))
        }
    }

    fn extract_field(json: &serde_json::Value, field_name: &str) -> Result<String, ServiceError> {
        json.get(field_name)
            .ok_or_else(|| ServiceError::InternalError(format!("Missing field: {}", field_name)))
            .and_then(|value| {
                value.as_str().ok_or_else(|| {
                    ServiceError::InternalError(format!("Invalid field: {}", field_name))
                })
            })
            .map(ToString::to_string)
    }

    pub async fn create_account(&self) -> Result<String, ServiceError> {
        let res = self
            .request(Method::POST, "/accounts", Some(vec![("type", "express")]))
            .await?;

        let json = Self::parse_response(res).await?;

        Self::extract_field(&json, "id")
    }

    pub async fn create_account_link(&self, account_id: &str) -> Result<String, ServiceError> {
        let res = self
            .request(
                Method::POST,
                "/account_links",
                Some(vec![
                    ("account", account_id),
                    ("refresh_url", &self.refresh_url),
                    ("return_url", &self.return_url),
                    ("type", "account_onboarding"),
                ]),
            )
            .await?;

        let json = Self::parse_response(res).await?;

        Self::extract_field(&json, "url")
    }
}

pub fn get_stripe_client() -> StripeClient {
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let base_url = std::env::var("BASE_URL").expect("Missing BASE_URL in env");
    let return_url = format!("{}/redirect/stripe/return", base_url);
    let refresh_url = format!("{}/redirect/stripe/refresh", base_url);
    let client = StripeClient::new(secret_key, return_url, refresh_url);

    client
}
