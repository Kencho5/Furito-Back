use sqlx::{Error, PgPool, Postgres};

pub async fn setup_database(pool: &PgPool) -> Result<(), Error> {
    // Check if the database exists
    let db_exists = sqlx::query("SELECT 1 FROM pg_database WHERE datname = 'furito'")
        .fetch_optional(pool)
        .await?
        .is_some();

    if !db_exists {
        // Create the database
        sqlx::query("CREATE DATABASE furito").execute(pool).await?;
    }

    // Connect to the furito database
    let furito_db_url = format!(
        "{}/furito",
        std::env::var("DB_URL").expect("DB_URL not set")
    );
    let furito_pool = PgPool::connect(&furito_db_url).await?;

    // Create tables if they don't exist
    let create_users_table = "
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username VARCHAR(50) NOT NULL,
            email VARCHAR(255) UNIQUE NOT NULL,
            password VARCHAR(255) NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
    ";
    let create_posts_table = "
        CREATE TABLE IF NOT EXISTS posts (
            id SERIAL PRIMARY KEY,
            user_id INT REFERENCES users(id),
            title VARCHAR(255) NOT NULL,
            content TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
    ";

    // Execute table creation
    sqlx::query(create_users_table)
        .execute(&furito_pool)
        .await?;
    sqlx::query(create_posts_table)
        .execute(&furito_pool)
        .await?;

    // Create index on users.email if it doesn't exist
    sqlx::query(
        "
        CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
    ",
    )
    .execute(&furito_pool)
    .await?;

    Ok(())
}
