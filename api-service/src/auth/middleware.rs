use std::sync::Arc;

use actix_web::{dev::ServiceRequest, error, web::Data, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use super::{jwt::validate_token, state::JwtValidatorState};

pub async fn jwt_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // Get the JwtValidatorState from the app data
    let state = req
        .app_data::<Data<Arc<JwtValidatorState>>>()
        .expect("Failed to get JwtValidatorState");
    let jwks = state.get_jwks().await;
    let token = credentials.token();

    // Validate the token
    return match validate_token(token, state.get_issuer(), jwks) {
        Ok(Some(user)) => {
            // Add user to the req data
            req.extensions_mut().insert(user);

            Ok(req)
        }
        _ => Err((Error::from(error::ErrorUnauthorized("Invalid token")), req)),
    };
}
