use sqlx::FromRow;
use serde::Serialize;

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub u_id: i64,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)] // Non esponiamo la password nell'API
    pub password: String,
}
