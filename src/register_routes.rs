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
        .layer(GovernorLayer {
            config: rate_limit!(2, 1),
        })
        .route("/register", post(auth::register::register_handler))
        .layer(GovernorLayer {
            config: rate_limit!(5, 1),
        })
}

fn verify_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/verify-email",
            post(verify::verify_email::verify_email_handler),
        )
        .layer(GovernorLayer {
            config: rate_limit!(60, 5),
        })
        .route(
            "/verify-email-code",
            post(verify::verify_email::verify_email_code_handler),
        )
        .layer(GovernorLayer {
            config: rate_limit!(2, 1),
        })
}

fn org_routes() -> Router<AppState> {
    Router::new()
        .route("/add-org", post(organizations::add_org::add_org_handler))
        .layer(GovernorLayer {
            config: rate_limit!(3, 1),
        })
}
