use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::super::client::StripeClient;
use super::super::error::StripeError;

use super::types::{
    AccountId, AccountLink, AccountRequirements, CreateAccountLink, CreateLoginLink, LoginLink,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Account {
    pub id: AccountId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<AccountRequirements>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_submitted: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub charges_enabled: Option<bool>,
}

impl Account {
    pub async fn create_account(client: &StripeClient) -> Result<Account, StripeError> {
        let account = client
            .request(Method::POST, "/accounts", Some(vec![("type", "express")]))
            .await?;

        Ok(account)
    }

    pub async fn create_account_link(
        client: &StripeClient,
        account_id: &AccountId,
    ) -> Result<AccountLink, StripeError> {
        let account_link = client
            .request(
                Method::POST,
                "/account_links",
                Some(CreateAccountLink {
                    account: account_id.clone(),
                    refresh_url: client.refresh_url.clone(),
                    return_url: client.return_url.clone(),
                    type_: "account_onboarding".to_string(),
                }),
            )
            .await?;

        Ok(account_link)
    }

    pub async fn create_login_link(
        client: &StripeClient,
        account_id: &AccountId,
    ) -> Result<LoginLink, StripeError> {
        let login_link = client
            .request(
                Method::POST,
                "/accounts/{}/login_links",
                Some(CreateLoginLink {
                    account: account_id.clone(),
                }),
            )
            .await?;

        Ok(login_link)
    }

    pub async fn get_account(
        client: &StripeClient,
        account_id: &AccountId,
    ) -> Result<Account, StripeError> {
        let res = client
            .request::<Account, ()>(
                Method::GET,
                &format!("/accounts/{}", account_id.as_str()),
                None,
            )
            .await?;

        Ok(res)
    }

    pub async fn is_onboarding_complete(
        client: &StripeClient,
        account_id: &AccountId,
    ) -> Result<bool, StripeError> {
        let account = Self::get_account(client, account_id).await?;

        let is_empty = match &account.requirements {
            Some(requirements) => match &requirements.currently_due {
                Some(currently_due) => currently_due.is_empty(),
                None => true,
            },
            None => true,
        };

        Ok(is_empty)
    }
}
