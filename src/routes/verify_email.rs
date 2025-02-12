use crate::email::insert_code::insert_code;
use crate::email::send_email::send_email;
use crate::prelude::*;

pub async fn verify_email_handler(
    State(state): State<AppState>,
    Json(payload): Json<VerifyEmailPayload>,
) -> Result<StatusCode, VerifyEmailError> {
    let code: u16 = rand::random_range(1000..9999);
    send_email(
        state.ses_client,
        &payload.email,
        "Verify email".to_string(),
        Some(code.to_string()),
    )
    .await
    .map_err(|_| VerifyEmailError::InvalidEmail)?;

    insert_code(state.pool, payload.email, code)
        .await
        .map_err(|_| VerifyEmailError::InsertFailed)?;

    Ok(StatusCode::OK)
}
