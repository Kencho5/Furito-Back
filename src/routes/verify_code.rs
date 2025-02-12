use crate::prelude::*;

pub async fn verify_code_handler(
    State(state): State<AppState>,
    Json(payload): Json<VerifyCodePayload>,
) -> Result<StatusCode, VerifyEmailError> {
    let result = sqlx::query_as::<_, EmailRecord>(
        "SELECT code, expiry FROM email_codes 
            WHERE email = $1 
            AND code = $2 
            AND used = FALSE
            ORDER BY created_at DESC
            LIMIT 1;",
    )
    .bind(&payload.email)
    .bind(&payload.code)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| VerifyEmailError::InvalidCode)?;

    match result {
        EmailRecord { code, expiry } if code == payload.code => {
            if chrono::Utc::now().naive_utc() > expiry {
                Err(VerifyEmailError::CodeExpired)
            } else {
                sqlx::query(
                    "UPDATE email_codes 
                     SET used = TRUE 
                     WHERE email = $1 AND code = $2;",
                )
                .bind(&payload.email)
                .bind(&payload.code)
                .execute(&state.pool)
                .await
                .map_err(|_| VerifyEmailError::InvalidCode)?;

                return Ok(StatusCode::OK);
            }
        }
        _ => return Err(VerifyEmailError::InvalidCode),
    }
}
