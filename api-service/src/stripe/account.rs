use stripe::{
    Account, AccountId, AccountLink, AccountLinkType, AccountType, Client, CreateAccount,
    CreateAccountLink, StripeError,
};

pub struct StripeAccount {}

impl StripeAccount {
    pub async fn create_account(client: &Client) -> Result<Account, StripeError> {
        let account = Account::create(
            &client,
            CreateAccount {
                type_: Some(AccountType::Express),
                ..Default::default()
            },
        )
        .await?;

        Ok(account)
    }

    pub async fn create_account_link(
        client: &Client,
        account_id: AccountId,
    ) -> Result<AccountLink, StripeError> {
        let base_url = std::env::var("BASE_URL").expect("Missing BASE_URL in env");
        let return_url = format!("{}/redirect/stripe/return", base_url);
        let refresh_url = format!("{}/redirect/stripe/refresh", base_url);
        let link = AccountLink::create(
            &client,
            CreateAccountLink {
                account: account_id,
                type_: AccountLinkType::AccountOnboarding,
                collect: None,
                expand: &[],
                refresh_url: Some(&return_url),
                return_url: Some(&refresh_url),
            },
        )
        .await?;

        Ok(link)
    }
}
