use alcoholic_jwt::JWKS;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct JwtValidatorState {
    issuer: String,
    jwks_uri: String,
    jwks: Arc<RwLock<Option<JWKS>>>,
}

impl JwtValidatorState {
    pub fn new(issuer: String, jwks_uri: String) -> Self {
        Self {
            issuer,
            jwks_uri,
            jwks: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn get_jwks(&self) -> JWKS {
        let jwks_guard = self.jwks.read().await;

        match &*jwks_guard {
            Some(jwks) => jwks.clone(),
            None => {
                drop(jwks_guard); // Release the lock before the async fetch

                let new_jwks = fetch_jwks(&self.jwks_uri).await;

                let mut jwks_guard = self.jwks.write().await;
                jwks_guard.get_or_insert(new_jwks.clone()).clone()
            }
        }
    }

    pub fn get_issuer(&self) -> String {
        self.issuer.clone()
    }
}

async fn fetch_jwks(uri: &str) -> JWKS {
    let res = reqwest::get(uri).await.expect("failed to fetch jwks");
    

    res.json::<JWKS>().await.expect("failed to parse jwks")
}
