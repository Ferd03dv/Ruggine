use serde::Serialize;
use sqlx::FromRow;

// Per ora accorpo User e UserGroup -> Spesso usati insieme
#[derive(Debug, FromRow, Serialize)]
pub struct Group {
    pub g_id: i64,
    pub name: String,
    pub created_by: i64, // id dell’utente creatore
}

#[derive(Debug, FromRow)]
pub struct UserGroup {
    pub ug_id: i64,
    pub user_id: i64,
    pub group_id: i64,
    pub is_admin: bool,
}

