use crate::{prelude::*, structs::org_struct::*};

pub async fn add_org_handler(
    State(state): State<AppState>,
    Json(payload): Json<AddOrgPayload>,
) -> Result<Json<AddOrgBody>, AddOrgError> {
    payload.validate()?;

    let presigned_url = put_object_url(
        &state.s3_client,
        "furito-assets",
        "organization-logos/test.jpg",
        60,
    )
    .await
    .map_err(|_| AddOrgError::Unforseen)?;

    Ok(Json(AddOrgBody::new(presigned_url)))
}
