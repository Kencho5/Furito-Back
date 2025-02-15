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
    token: String,
}

pub enum RegisterError {
    MissingCredentials,
    EmailTaken,
    TokenCreation,
}

impl RegisterBody {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

impl IntoResponse for RegisterError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            RegisterError::MissingCredentials => {
                (StatusCode::UNAUTHORIZED, "AUTH.ERROR.missing_credentials")
            }
            RegisterError::EmailTaken => (StatusCode::BAD_REQUEST, "AUTH.ERROR.email_taken"),
            RegisterError::TokenCreation => {
                (StatusCode::INTERNAL_SERVER_ERROR, "AUTH.ERROR.unforseen")
            }
        };
        let body = Json(json!({
            "message": error_message,
        }));
        (status, body).into_response()
    }
}

impl RegisterPayload {
    pub fn validate(&self) -> Result<(), RegisterError> {
        let json_value = serde_json::to_value(self).unwrap();

        if let Value::Object(map) = json_value {
            for (_, value) in map.iter() {
                if value.as_str().map_or(true, |s| s.is_empty()) {
                    return Err(RegisterError::MissingCredentials);
                }
            }
        }

        Ok(())
    }
}
