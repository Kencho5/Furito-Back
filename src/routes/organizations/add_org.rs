use crate::{prelude::*, structs::org_struct::AddOrgError};

pub async fn add_org_handler(
    State(state): State<AppState>,
    Json(payload): Json<VerifyEmailPayload>,
) -> Result<StatusCode, AddOrgError> {
    Ok(StatusCode::OK)
}
