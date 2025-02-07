use crate::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct RegisterPayload {
    pub name: String,
    pub surname: String,
    pub email: String,
    pub phone_code: String,
    pub phone: String,
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
            RegisterError::MissingCredentials => {
                (StatusCode::UNAUTHORIZED, "AUTH.ERROR.missing_credentials")
            }
            RegisterError::EmailTaken => (StatusCode::BAD_REQUEST, "AUTH.ERROR.email_taken"),
        };
        let body = Json(json!({
            "message": error_message,
        }));
        (status, body).into_response()
    }
}

impl RegisterPayload {
    pub fn validate(&self) -> Result<(), RegisterError> {
        let required_fields = [
            self.name.as_str(),
            self.surname.as_str(),
            self.email.as_str(),
            self.phone_code.as_str(),
            self.phone.as_str(),
            self.password.as_str(),
        ];

        for value in required_fields {
            if value.is_empty() {
                return Err(RegisterError::MissingCredentials);
            }
        }

        Ok(())
    }
}
