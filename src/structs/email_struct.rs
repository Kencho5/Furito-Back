use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct VerifyEmailPayload {
    pub email: String,
}

pub enum VerifyEmailError {
    InvalidEmail,
}

impl IntoResponse for VerifyEmailError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            VerifyEmailError::InvalidEmail => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "AUTH.ERROR.invalid_email",
            ),
        };
        let body = Json(json!({
            "message": error_message,
        }));
        (status, body).into_response()
    }
}
