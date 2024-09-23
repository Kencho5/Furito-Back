use crate::prelude::*;

pub async fn login_handler(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let users = sqlx::query_as::<_, User>("SELECT id, name FROM users")
        .fetch_all(&pool)
        .await
        .map_err(internal_error)?;

    Ok(Json(users))
}
