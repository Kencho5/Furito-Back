mod email;
mod prelude;
mod rate_limit;
mod register_routes;
mod routes;
mod structs;
mod utils;

use crate::prelude::*;
use tracing::Level;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let log_level = if env::var("ENVIRONMENT").expect("ENVIRONMENT not set") == "staging" {
        Level::DEBUG
    } else {
        Level::INFO
    };

    tracing_subscriber::fmt().with_max_level(log_level).init();

    let frontend_url = env::var("FRONTEND_URL").expect("Frontend url not set");
    let db_url = env::var("DB_URL").expect("DB url not set");
    let port = env::var("PORT").expect("Port not set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to the database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let aws_config = aws_config::load_from_env().await;
    let ses_client = aws_sdk_sesv2::Client::new(&aws_config);
    let s3_client = aws_sdk_s3::Client::new(&aws_config);

    let state = AppState {
        pool,
        ses_client,
        s3_client,
    };

    let app = register_routes::create_router()
        .layer(
            CorsLayer::new()
                .allow_origin(frontend_url.parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                .allow_headers([http::header::CONTENT_TYPE, http::header::AUTHORIZATION]),
        )
        .with_state(state);

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
