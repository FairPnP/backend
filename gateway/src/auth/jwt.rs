use alcoholic_jwt::{token_kid, validate, ValidJWT, Validation, ValidationError, JWKS};

pub fn validate_token(
    token: &str,
    issuer: String,
    jwks: JWKS,
) -> Result<ValidJWT, ValidationError> {
    let validations: Vec<Validation> = vec![Validation::Issuer(issuer), Validation::SubjectPresent];

    let kid = token_kid(token)?.expect("failed to find kid");
    let jwk = jwks.find(&kid).expect("failed to find jwk");

    validate(token, jwk, validations)
}
