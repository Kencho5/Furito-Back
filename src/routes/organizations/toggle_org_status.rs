use crate::{prelude::*, structs::org_struct::*};

pub async fn toggle_org_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<ToggleOrgStatusPayload>,
) -> Result<StatusCode, OrgsError> {
    let claims = validate_token(headers)
        .await
        .map_err(|_| OrgsError::Unauthorized)?;

    sqlx::query("UPDATE organizations SET enabled = not enabled WHERE id = $1 AND owner = $2")
        .bind(&payload.id)
        .bind(&claims.email)
        .execute(&state.pool)
        .await
        .map_err(|_| OrgsError::Unauthorized)?;

    Ok(StatusCode::OK)
}
