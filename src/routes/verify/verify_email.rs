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

pub async fn verify_email_code_handler(
    State(state): State<AppState>,
    Json(payload): Json<VerifyCodePayload>,
) -> Result<StatusCode, VerifyEmailError> {
    let result = sqlx::query_as::<_, EmailRecord>(
        "SELECT code, expiry FROM email_codes 
            WHERE email = $1 
            AND code = $2 
            LIMIT 1;",
    )
    .bind(&payload.email)
    .bind(&payload.code)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| VerifyEmailError::InvalidCode)?;

    match result {
        EmailRecord { code, expiry } if code == payload.code => {
            let _ = sqlx::query(
                "DELETE FROM email_codes 
                     WHERE email = $1 AND code = $2;",
            )
            .bind(&payload.email)
            .bind(&payload.code)
            .execute(&state.pool)
            .await;

            if chrono::Utc::now().naive_utc() > expiry {
                Err(VerifyEmailError::CodeExpired)
            } else {
                return Ok(StatusCode::OK);
            }
        }
        _ => return Err(VerifyEmailError::InvalidCode),
    }
}
