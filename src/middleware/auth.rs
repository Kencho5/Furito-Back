use crate::prelude::*;
use crate::utils::validate_token::validate_token;
use axum::extract::Request;
use axum::middleware::Next;

pub async fn validate_headers(req: Request, next: Next) -> Result<Response, StatusCode> {
    let token = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    if token.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    validate_token(token.unwrap())
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(next.run(req).await)
}
