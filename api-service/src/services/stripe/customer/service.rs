#![allow(dead_code)]
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::services::stripe::account::types::AccountId;

use super::super::client::StripeClient;
use super::super::error::StripeError;

use super::types::{CreatePaymentIntent, CustomerId, EphemeralKey, PaymentIntent};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Customer {
    pub id: CustomerId,
}

impl Customer {
    pub async fn create_customer(client: &StripeClient) -> Result<Customer, StripeError> {
        let customer = client
            .request::<Customer, ()>(Method::POST, "/customers", None)
            .await?;

        Ok(customer)
    }

    pub async fn get_customer(
        client: &StripeClient,
        customer_id: &CustomerId,
    ) -> Result<Customer, StripeError> {
        let res = client
            .request::<Customer, ()>(
                Method::GET,
                &format!("/customers/{}", customer_id.as_str()),
                None,
            )
            .await?;

        Ok(res)
    }

    pub async fn create_ephemeral_key(
        client: &StripeClient,
        customer_id: &CustomerId,
    ) -> Result<EphemeralKey, StripeError> {
        let res = client
            .request(
                Method::POST,
                "/ephemeral_keys",
                Some(vec![("customer", customer_id.as_str())]),
            )
            .await?;

        Ok(res)
    }

    pub async fn create_payment_intent(
        client: &StripeClient,
        customer_id: &CustomerId,
        dest_account_id: &AccountId,
        amount: i64,
        currency: &str,
        platform_fee: i64,
    ) -> Result<PaymentIntent, StripeError> {
        let res = client
            .request(
                Method::POST,
                "/payment_intents",
                Some(CreatePaymentIntent {
                    amount,
                    currency: currency.to_string(),
                    customer: customer_id.clone(),
                    application_fee_amount: Some(platform_fee),
                    transfer_destination: Some(dest_account_id.clone()),
                }),
            )
            .await?;

        Ok(res)
    }
}
