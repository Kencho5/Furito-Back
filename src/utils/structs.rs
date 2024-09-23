use serde::Serialize;

#[derive(Serialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub phone: String,
    pub city: String,
    pub facebook: String,
    pub password: String,
}
