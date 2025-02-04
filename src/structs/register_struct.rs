use crate::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct RegisterPayload {
    pub email: String,
    pub registration_type: String,
    pub phone: String,
    pub phone_code: String,
    pub password: String,

    // OPTIONAL
    pub name: Option<String>,
    pub company_code: Option<String>,
    pub company_name: Option<String>,
    pub terms: Option<String>,
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
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

impl RegisterPayload {
    pub fn validate(&self) -> Result<(), RegisterError> {
        if self.email.is_empty() {
            return Err(RegisterError::MissingCredentials);
        }
        if self.registration_type.is_empty() {
            return Err(RegisterError::MissingCredentials);
        }
        if self.phone.is_empty() {
            return Err(RegisterError::MissingCredentials);
        }
        if self.phone_code.is_empty() {
            return Err(RegisterError::MissingCredentials);
        }
        if self.password.is_empty() {
            return Err(RegisterError::MissingCredentials);
        }

        Ok(())
    }
}
