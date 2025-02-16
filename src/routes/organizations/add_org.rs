use crate::{prelude::*, structs::org_struct::*};

pub async fn add_org_handler(
    State(state): State<AppState>,
    Json(payload): Json<AddOrgPayload>,
) -> Result<Json<AddOrgBody>, AddOrgError> {
    payload.validate()?;

    let file_name = format!("{}-{}", payload.org_name, payload.org_code);
    let presigned_url = put_object_url(
        &state.s3_client,
        "furito-assets",
        format!(
            "organization-logos-{}/{}.jpg",
            env::var("ENVIRONMENT").expect("ENVIRONMENT not set"),
            file_name
        )
        .as_str(),
        60,
    )
    .await
    .map_err(|_| AddOrgError::Unforseen)?;

    sqlx::query(
        "INSERT INTO organizations (email, org_code, org_type, org_name, address, phone_code, phone) VALUES($1, $2, $3, $4, $5, $6, $7)",
    )
    .bind(&payload.email)
    .bind(&payload.org_code)
    .bind(&payload.org_type)
    .bind(&payload.org_name)
    .bind(&payload.address)
    .bind(&payload.phone_code)
    .bind(&payload.phone)
    .execute(&state.pool)
    .await
    .map_err(|_| AddOrgError::Unforseen)?;

    Ok(Json(AddOrgBody::new(presigned_url)))
}
