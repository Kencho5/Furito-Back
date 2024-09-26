use sqlx::{Error, PgPool};

use std::fs;

pub async fn setup_database(pool: &PgPool) -> Result<(), Error> {
    let sql = fs::read_to_string("src/utils/schema.sql").expect("Failed to read schema.sql");

    sqlx::query(&sql).execute(pool).await?;

    Ok(())
}
