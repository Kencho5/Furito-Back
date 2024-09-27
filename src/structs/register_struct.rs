use crate::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct RegisterPayload {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterBody {
    message: String,
}

pub enum RegisterError {
    MissingCredentials,
    EmailTaken,
}

impl RegisterBody {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl IntoResponse for RegisterError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            RegisterError::MissingCredentials => (StatusCode::UNAUTHORIZED, "Missing credentials"),
            RegisterError::EmailTaken => (StatusCode::BAD_REQUEST, "Email taken"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
