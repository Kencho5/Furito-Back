mod register_routes;
mod routes;

#[tokio::main]
async fn main() {
    let app = register_routes::create_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
