use crate::prelude::*;
use crate::routes::*;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .merge(auth_routes())
        .merge(verify_routes())
        .merge(org_routes())
        .layer(
            RateLimitLayer::<RealIp>::builder()
                .with_gc_interval(1000)
                .with_gc_interval(std::time::Duration::from_secs(60))
                .default_handle_error(),
        )
}

fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(auth::login::login_handler))
        .route_layer(rate_limit!(2))
        //.route_layer(middleware::from_fn(validate_headers))
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
