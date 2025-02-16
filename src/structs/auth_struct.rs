use crate::prelude::*;

#[derive(Serialize, sqlx::FromRow)]
pub struct User {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct AuthPayload {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthBody {
    token: String,
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub email: String,
    pub company: String,
    pub exp: i64,
}

impl AuthBody {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => {
                (StatusCode::UNAUTHORIZED, "AUTH.ERROR.wrong_credentials")
            }
            AuthError::MissingCredentials => {
                (StatusCode::BAD_REQUEST, "AUTH.ERROR.missing_credentials")
            }
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "AUTH.ERROR.unforseen"),
            AuthError::InvalidToken => (StatusCode::INTERNAL_SERVER_ERROR, "AUTH.ERROR.unforseen"),
        };
        let body = Json(json!({
            "message": error_message,
        }));
        (status, body).into_response()
    }
}
