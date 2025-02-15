use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct AddOrgPayload {
    pub email: String,
}

pub enum AddOrgError {
    InvalidEmail,
}

impl IntoResponse for AddOrgError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AddOrgError::InvalidEmail => (
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
