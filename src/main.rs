mod prelude;
mod register_routes;
mod routes;
mod structs;
mod utils;

use crate::prelude::*;
use crate::utils::setup_db::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let frontend_url = env::var("FRONTEND_URL").expect("Frontend url not set");
    let db_url = env::var("DB_URL").expect("DB url not set");
    let port = env::var("PORT").expect("Port not set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to the database");

    let _ = setup_database(&pool).await;

    let app = register_routes::create_router()
        .layer(
            CorsLayer::new()
                .allow_origin(frontend_url.parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([http::header::CONTENT_TYPE]),
        )
        .with_state(pool);

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
