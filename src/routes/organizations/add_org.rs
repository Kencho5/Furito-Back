use crate::{prelude::*, structs::org_struct::*};

pub async fn add_org_handler(
    State(state): State<AppState>,
    Json(payload): Json<AddOrgPayload>,
) -> Result<StatusCode, AddOrgError> {
    payload.validate()?;

    Ok(StatusCode::OK)
}
