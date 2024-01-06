use stripe::Client;

pub mod account;

pub fn get_stripe_client() -> Client {
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = Client::new(secret_key);

    client
}
