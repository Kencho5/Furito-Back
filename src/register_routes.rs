use crate::prelude::*;
use crate::routes::*;

pub fn create_router() -> Router<PgPool> {
    Router::new().merge(auth_routes())
}

fn auth_routes() -> Router<PgPool> {
    Router::new().route("/login", post(login::login_handler))
}
