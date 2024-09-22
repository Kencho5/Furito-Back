use crate::routes::*;
use axum::routing::post;
use axum::Router;

pub fn create_router() -> Router {
    Router::new().merge(auth_routes())
}

fn auth_routes() -> Router {
    Router::new().route("/api/login", post(login::login_handler))
}
