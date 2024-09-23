use sqlx::{Error, PgPool, Postgres};

pub async fn setup_database(pool: &PgPool) -> Result<(), Error> {
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
    sqlx::query(create_users_table).execute(pool).await?;
    sqlx::query(create_posts_table).execute(pool).await?;

    // Create index on users.email if it doesn't exist
    sqlx::query(
        "
        CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
    ",
    )
    .execute(pool)
    .await?;

    Ok(())
}
