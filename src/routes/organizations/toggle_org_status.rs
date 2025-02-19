use crate::{prelude::*, structs::org_struct::*};

pub async fn toggle_org_handler(
    State(state): State<AppState>,
    Json(payload): Json<ToggleOrgStatusPayload>,
) -> Result<StatusCode, OrgsError> {
    sqlx::query("UPDATE organizations SET enabled = not enabled WHERE id = $1")
        .bind(&payload.id)
        .execute(&state.pool)
        .await
        .map_err(|_| OrgsError::Unforseen)?;

    Ok(StatusCode::OK)
}
