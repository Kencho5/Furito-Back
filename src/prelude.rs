pub use crate::utils::structs::*;
pub use axum::http::{HeaderValue, Method, StatusCode};
pub use axum::{extract::State, routing::post, Json, Router};
pub use dotenv::dotenv;
pub use sqlx::postgres::{PgPool, PgPoolOptions};
pub use std::env;
pub use tokio::net::TcpListener;
pub use tower_http::cors::CorsLayer;

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
