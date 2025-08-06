use sqlx::SqlitePool;
use crate::models::User;

// POST: Inserisci uno User all'interno della tabella user
pub async fn insert_user(pool: &SqlitePool, username: &str, email: &str, password: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO user (username, email, password) VALUES (?, ?, ?)")
        .bind(username)
        .bind(email)
        .bind(password)
        .execute(pool)
        .await?;
    Ok(())
}

// GET: User appartenente a un certo ID
pub async fn get_user_by_id(pool: &SqlitePool, id: i64) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM user WHERE u_id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}

// GET: Lista di tutti gli users (Vedi componenti Gruppo)
pub async fn get_all_users(pool: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM user")
        .fetch_all(pool)
        .await
}
