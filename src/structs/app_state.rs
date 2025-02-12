use crate::prelude::*;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub ses_client: aws_sdk_sesv2::Client,
}
