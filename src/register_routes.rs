use crate::prelude::*;
use crate::routes::*;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .merge(auth_routes())
        .merge(verify_routes())
        .merge(org_routes())
}

fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(auth::login::login_handler))
        .route("/register", post(auth::register::register_handler))
}

fn verify_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/verify-email",
            post(verify::verify_email::verify_email_handler),
        )
        .route(
            "/verify-email-code",
            post(verify::verify_email::verify_email_code_handler),
        )
}

fn org_routes() -> Router<AppState> {
    Router::new().route("/add-org", post(organizations::add_org::add_org_handler))
}
