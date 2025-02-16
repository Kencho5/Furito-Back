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
        format!("organization-logos/{}.jpg", file_name).as_str(),
        60,
    )
    .await
    .map_err(|_| AddOrgError::Unforseen)?;

    Ok(Json(AddOrgBody::new(presigned_url)))
}
