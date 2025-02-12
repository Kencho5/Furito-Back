use crate::prelude::*;
use chrono::Duration;
use sqlx::types::chrono::Utc;
use sqlx::Error;

pub async fn insert_code(pool: PgPool, email: String, code: u16) -> Result<(), Error> {
    sqlx::query("INSERT INTO email_codes(email, code, expiry) VALUES($1, $2, $3)")
        .bind(&email)
        .bind(code as i32)
        .bind(Utc::now() + Duration::minutes(5))
        .execute(&pool)
        .await?;

    Ok(())
}
