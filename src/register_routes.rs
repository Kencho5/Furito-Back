use crate::prelude::*;
use crate::routes::*;

pub fn create_router() -> Router<AppState> {
    Router::new().merge(auth_routes())
}

fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login::login_handler))
        .route("/register", post(register::register_handler))
        .route(
            "/verify-email",
            post(verify::verify_email::verify_email_handler),
        )
        .route(
            "/verify-email-code",
            post(verify::verify_email::verify_email_code_handler),
        )
}
