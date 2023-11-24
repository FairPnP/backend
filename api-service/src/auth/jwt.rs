use actix_web::Error;
use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use uuid::Uuid;

use crate::users::User;

pub fn validate_token(token: &str, issuer: String, jwks: JWKS) -> Result<Option<User>, Error> {
    let validations: Vec<Validation> = vec![Validation::Issuer(issuer), Validation::SubjectPresent];
    let kid = match token_kid(&token) {
        Ok(res) => res.expect("failed to decode kid"),
        Err(_) => return Ok(None),
    };

    let jwk = jwks.find(&kid).expect("Specified key not found in set");
    let res = validate(token, jwk, validations);

    match res {
        Ok(jwt) => {
            let user: User = User {
                id: Uuid::parse_str(
                    jwt.claims["sub"]
                        .as_str()
                        .expect("failed to parse sub claim"),
                )
                .expect("failed to parse uuid"),
                client_id: jwt.claims["client_id"].to_string(),
                username: jwt.claims["username"].to_string(),
            };
            return Ok(Some(user));
        }
        Err(_) => return Ok(None),
    };
}
