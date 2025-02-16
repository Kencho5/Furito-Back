use crate::{
    prelude::*,
    structs::auth_struct::{AuthError, Claims},
};

pub async fn validate_token(token: &str) -> Result<Claims, AuthError> {
    let key = env::var("SECRET_KEY").expect("Secret key not set");
    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(key.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| AuthError::InvalidToken)?;

    let expiry = DateTime::from_timestamp_millis(token_data.claims.exp).unwrap();
    if expiry <= Utc::now() {
        return Err(AuthError::InvalidToken);
    }

    Ok(token_data.claims)
}
