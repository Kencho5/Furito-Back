use crate::prelude::*;
use axum::middleware::Next;

pub async fn validate_headers(
    headers: HeaderMap,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    validate_token(headers)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(next.run(req).await)
}
