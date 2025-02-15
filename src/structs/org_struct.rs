use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddOrgPayload {
    pub logo: String,
    pub org_code: String,
    pub email: String,
    pub org_type: String,
    pub org_name: String,
    pub address: String,
    pub phone: String,
    pub phone_code: String,
}

pub enum AddOrgError {
    MissingCredentials,
}

impl IntoResponse for AddOrgError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AddOrgError::MissingCredentials => {
                (StatusCode::UNAUTHORIZED, "AUTH.ERROR.missing_credentials")
            }
        };
        let body = Json(json!({
            "message": error_message,
        }));
        (status, body).into_response()
    }
}

impl AddOrgPayload {
    pub fn validate(&self) -> Result<(), AddOrgError> {
        let json_value = serde_json::to_value(self).unwrap();

        if let Value::Object(map) = json_value {
            for (_, value) in map.iter() {
                if value.as_str().map_or(true, |s| s.is_empty()) {
                    return Err(AddOrgError::MissingCredentials);
                }
            }
        }

        Ok(())
    }
}
