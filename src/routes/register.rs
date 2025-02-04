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
    payload.validate()?;

    let pwh = bcrypt::hash(&payload.password).unwrap();
    sqlx::query(
        "INSERT INTO users(email, name, phone, phone_code, password) VALUES($1, $2, $3, $4, $5)",
    )
    .bind(&payload.email)
    .bind(&payload.name)
    .bind(&payload.phone)
    .bind(&payload.phone_code)
    .bind(&pwh)
    .execute(pool)
    .await
    .map_err(|_| RegisterError::EmailTaken)?;

    Ok(())
}
