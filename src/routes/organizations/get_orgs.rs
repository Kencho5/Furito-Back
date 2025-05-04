use crate::{prelude::*, structs::org_struct::*};

pub async fn get_orgs_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<OrgsResponse>, OrgsError> {
    let claims = validate_token(headers)
        .await
        .map_err(|_| OrgsError::Unauthorized)?;

    let orgs = sqlx::query_as::<_, Org>(
        "SELECT * FROM organizations WHERE owner = $1 ORDER BY created_at DESC",
    )
    .bind(&claims.email)
    .fetch_all(&state.pool)
    .await
    .map_err(|_| OrgsError::EmptyOrgs)?;

    Ok(Json(OrgsResponse::new(orgs)))
}

pub async fn get_org_handler(
    State(state): State<AppState>,
    Path(org_id): Path<i16>,
) -> Result<Json<Org>, OrgsError> {
    let org = sqlx::query_as::<_, Org>(
        "SELECT * FROM organizations WHERE id = $1 ORDER BY created_at DESC",
    )
    .bind(&org_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| OrgsError::Unforseen)?;

    Ok(Json(org))
}
