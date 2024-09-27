use crate::prelude::*;
use crate::structs::auth_struct::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn login_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthBody>, AuthError> {
    validate_credentials(&payload, &pool).await?;
    let token = create_token(payload.email).await?;

    Ok(Json(AuthBody::new(token)))
}

async fn validate_credentials(payload: &AuthPayload, pool: &PgPool) -> Result<(), AuthError> {
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    let user = sqlx::query_as::<_, User>("SELECT email, password FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_one(pool)
        .await
        .map_err(|_| AuthError::WrongCredentials)?;

    if payload.email != user.email || !bcrypt::verify(&payload.password, &user.password) {
        return Err(AuthError::WrongCredentials);
    }

    Ok(())
}

async fn create_token(email: String) -> Result<String, AuthError> {
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u64
        + 7 * 24 * 60 * 60;

    let claims = Claims {
        sub: email,
        company: "Furito".to_owned(),
        exp,
    };

    let key = env::var("SECRET_KEY").expect("Secret key not set");
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(key.as_ref()),
    )
    .map_err(|_| AuthError::TokenCreation)
}
