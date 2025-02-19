use crate::prelude::*;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct AddOrgPayload {
    pub org_code: String,
    pub email: String,
    pub org_type: String,
    pub org_name: String,
    pub address: String,
    pub phone: String,
    pub phone_code: String,
}

#[derive(Serialize)]
pub struct AddOrgBody {
    presigned_url: String,
}

impl AddOrgBody {
    pub fn new(presigned_url: String) -> Self {
        Self { presigned_url }
    }
}

pub enum OrgsError {
    MissingCredentials,
    Unforseen,
    Unauthorized,
    EmptyOrgs,
}

impl IntoResponse for OrgsError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            OrgsError::MissingCredentials => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "AUTH.ERROR.missing_credentials",
            ),
            OrgsError::Unforseen => (StatusCode::INTERNAL_SERVER_ERROR, "AUTH.ERROR.unforseen"),
            OrgsError::Unauthorized => (StatusCode::UNAUTHORIZED, "AUTH.ERROR.unauthorized"),
            OrgsError::EmptyOrgs => (StatusCode::NOT_FOUND, "empty orgs"),
        };
        let body = Json(json!({
            "message": error_message,
        }));
        (status, body).into_response()
    }
}

impl AddOrgPayload {
    pub fn validate(&self) -> Result<(), OrgsError> {
        let json_value = serde_json::to_value(self).unwrap();

        if let Value::Object(map) = json_value {
            for (_, value) in map.iter() {
                if value.as_str().map_or(true, |s| s.is_empty()) {
                    return Err(OrgsError::MissingCredentials);
                }
            }
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct OrgsPayload {
    pub id: i32,
    pub org_code: String,
    pub email: String,
    pub org_type: String,
    pub org_name: String,
    pub address: String,
    pub phone: String,
    pub phone_code: String,
    pub enabled: bool,
}

#[derive(Serialize)]
pub struct OrgsResponse {
    pub orgs: Vec<OrgsPayload>,
    pub total: i16,
}

impl OrgsResponse {
    pub fn new(orgs: Vec<OrgsPayload>) -> Self {
        Self {
            total: orgs.len() as i16,
            orgs,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ToggleOrgStatusPayload {
    pub id: i32,
}
