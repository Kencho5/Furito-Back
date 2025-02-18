use crate::{prelude::*, structs::org_struct::*};

pub async fn get_orgs_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<OrgsResponse>, OrgsError> {
    let token = extract_token(headers).unwrap();
    let claims = validate_token(&token)
        .await
        .map_err(|_| OrgsError::Unauthorized)?;

    let orgs = sqlx::query_as::<_, OrgsPayload>("SELECT * FROM organizations WHERE owner = $1")
        .bind(&claims.email)
        .fetch_all(&state.pool)
        .await
        .map_err(|_| OrgsError::EmptyOrgs)?;

    Ok(Json(OrgsResponse::new(orgs)))
}
