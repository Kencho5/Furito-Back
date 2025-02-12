use crate::email::send_email::send_email;
use crate::prelude::*;
use crate::structs::email_struct::*;

pub async fn verify_email_handler(
    State(state): State<AppState>,
    Json(payload): Json<VerifyEmailPayload>,
) -> Result<StatusCode, VerifyEmailError> {
    let code: u16 = rand::random_range(1000..9999);
    send_email(
        state.ses_client,
        payload.email,
        "Test".to_string(),
        Some(code.to_string()),
    )
    .await;

    Ok(StatusCode::OK)
}
