use crate::prelude::*;

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthBody>, AuthError> {
    validate_credentials(&payload, &state.pool).await?;
    let token = create_token(payload.email).await?;

    Ok(Json(AuthBody::new(token)))
}

async fn validate_credentials(payload: &AuthPayload, pool: &PgPool) -> Result<(), AuthError> {
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    let user = sqlx::query_as::<_, User>("SELECT email, password FROM users WHERE email = $1")
        .bind(&payload.email.to_lowercase())
        .fetch_one(pool)
        .await
        .map_err(|_| AuthError::WrongCredentials)?;

    if payload.email != user.email || !bcrypt::verify(&payload.password, &user.password) {
        return Err(AuthError::WrongCredentials);
    }

    Ok(())
}

pub async fn create_token(email: String) -> Result<String, AuthError> {
    let exp = Utc::now() + Duration::days(7);

    let claims = Claims {
        email,
        company: "Furito LLC".to_string(),
        exp: exp.timestamp_millis(),
    };

    let key = env::var("SECRET_KEY").expect("Secret key not set");
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(key.as_ref()),
    )
    .map_err(|_| AuthError::TokenCreation)
}
