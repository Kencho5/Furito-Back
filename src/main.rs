mod prelude;
mod register_routes;
mod routes;

use crate::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let frontend_url = std::env::var("FRONTEND_URL").expect("Frontend url not set");

    let app = register_routes::create_router().layer(
        CorsLayer::new()
            .allow_origin(frontend_url.parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET, Method::POST])
            .allow_headers([http::header::CONTENT_TYPE]),
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
