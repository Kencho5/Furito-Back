use crate::prelude::*;
use sqlx::Error;

pub async fn insert_code(pool: PgPool, email: String, code: u16) -> Result<(), Error> {
    sqlx::query(
        "INSERT INTO email_codes(email, code, expiry) VALUES($1, $2, NOW() + INTERVAL '0 minutes')",
    )
    .bind(&email)
    .bind(code as i32)
    .execute(&pool)
    .await?;

    Ok(())
}
