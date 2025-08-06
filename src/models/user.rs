use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct User {
    pub u_id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
}
