use crate::prelude::*;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub ses_client: aws_sdk_sesv2::Client,
    pub s3_client: aws_sdk_s3::Client,
}
