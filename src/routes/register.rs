use crate::prelude::*;
use crate::structs::register_struct::*;

pub async fn register_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<RegisterBody>, RegisterError> {
    insert_credentials(&payload, &pool).await?;

    Ok(Json(RegisterBody::new("Success".to_string())))
}

async fn insert_credentials(payload: &RegisterPayload, pool: &PgPool) -> Result<(), RegisterError> {
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(RegisterError::MissingCredentials);
    }

    let pwh = bcrypt::hash(&payload.password).unwrap();
    sqlx::query("INSERT INTO users(email, password) VALUES($1, $2)")
        .bind(&payload.email)
        .bind(&pwh)
        .execute(pool)
        .await
        .map_err(|_| RegisterError::EmailTaken)?;

    Ok(())
}
