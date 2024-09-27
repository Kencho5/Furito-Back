use sqlx::{Error, PgPool};
use std::fs;

pub async fn setup_database(pool: &PgPool) -> Result<(), Error> {
    let sql_files = vec!["users", "posts", "indexes"];

    for file in sql_files {
        execute_sql(pool, file).await?;
    }
    Ok(())
}

async fn execute_sql(pool: &PgPool, file: &str) -> Result<(), Error> {
    let schema = fs::read_to_string(format!("src/schemas/{}.sql", file))
        .expect(format!("Unable to read {}.sql", file).as_str());
    sqlx::query(&schema).execute(pool).await?;

    Ok(())
}
