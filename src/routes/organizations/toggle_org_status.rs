use crate::{prelude::*, structs::org_struct::*};

pub async fn toggle_org_handler(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(payload): Json<ToggleOrgStatusPayload>,
) -> Result<StatusCode, OrgsError> {
    validate_token(headers)
        .await
        .map_err(|_| OrgsError::Unauthorized)?;

    sqlx::query("UPDATE organizations SET enabled = not enabled WHERE id = $1")
        .bind(&payload.id)
        .execute(&state.pool)
        .await
        .map_err(|_| OrgsError::Unforseen)?;

    Ok(StatusCode::OK)
}
