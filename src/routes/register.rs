use crate::prelude::*;
use crate::routes::login::create_token;
use crate::structs::register_struct::*;

pub async fn register_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<RegisterBody>, RegisterError> {
    insert_credentials(&payload, &pool).await?;

    let token = create_token(payload.email.clone())
        .await
        .map_err(|_| RegisterError::TokenCreation)?;

    Ok(Json(RegisterBody::new(token)))
}

async fn insert_credentials(payload: &RegisterPayload, pool: &PgPool) -> Result<(), RegisterError> {
    payload.validate()?;

    let pwh = bcrypt::hash(&payload.password).unwrap();
    sqlx::query(
        "INSERT INTO users(name, surname, email, phone, phone_code, password) VALUES($1, $2, $3, $4, $5, $6)",
    )
    .bind(&payload.name)
    .bind(&payload.surname)
    .bind(&payload.email)
    .bind(&payload.phone)
    .bind(&payload.phone_code)
    .bind(&pwh)
    .execute(pool)
    .await
    .map_err(|_| RegisterError::EmailTaken)?;

    Ok(())
}
