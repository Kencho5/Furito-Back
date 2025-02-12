use chrono::NaiveDateTime;

use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct VerifyEmailPayload {
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct VerifyCodePayload {
    pub email: String,
    pub code: i16,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct EmailRecord {
    pub code: i16,
    pub expiry: NaiveDateTime,
}

pub enum VerifyEmailError {
    InvalidEmail,
    InsertFailed,
    InvalidCode,
    CodeExpired,
}

impl IntoResponse for VerifyEmailError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            VerifyEmailError::InvalidEmail => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "AUTH.ERROR.invalid_email",
            ),
            VerifyEmailError::InsertFailed => {
                (StatusCode::INTERNAL_SERVER_ERROR, "AUTH.ERROR.unforseen")
            }
            VerifyEmailError::InvalidCode => {
                (StatusCode::INTERNAL_SERVER_ERROR, "AUTH.ERROR.invalid_code")
            }
            VerifyEmailError::CodeExpired => {
                (StatusCode::INTERNAL_SERVER_ERROR, "AUTH.ERROR.code_expired")
            }
        };
        let body = Json(json!({
            "message": error_message,
        }));
        (status, body).into_response()
    }
}
