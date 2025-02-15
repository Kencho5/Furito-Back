pub use crate::structs::app_state::AppState;
pub use crate::structs::email_struct::*;
pub use axum::{
    extract::State,
    http::{HeaderValue, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
pub use dotenv::dotenv;
pub use jsonwebtoken::{encode, EncodingKey, Header};
pub use pwhash::bcrypt;
pub use serde::{Deserialize, Serialize};
pub use serde_json::{json, Value};
pub use sqlx::postgres::{PgPool, PgPoolOptions};
pub use std::env;
pub use tokio::net::TcpListener;
pub use tower_http::cors::CorsLayer;
