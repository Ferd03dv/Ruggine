use sqlx::{SqlitePool, query, query_as};
use crate::models::group::Group;

pub async fn create_group(pool: &SqlitePool, name: &str, created_by: i64) -> Result<i64, sqlx::Error> {
    let res = query("INSERT INTO groups (name, created_by) VALUES (?, ?)")
        .bind(name)
        .bind(created_by)
        .execute(pool)
        .await?;

    let group_id = res.last_insert_rowid();

    query("INSERT INTO user_group (user_id, group_id, is_admin) VALUES (?, ?, 1)")
        .bind(created_by)
        .bind(group_id)
        .execute(pool)
        .await?;
    Ok(group_id)
}

// GET: Restituisco tutti i gruppi di cui fa parte un utente!
pub async fn list_groups_by_user(pool: &SqlitePool, user_id: i64) -> Result<Vec<Group>, sqlx::Error> {
    query_as::<_, Group>(
        r#"
        SELECT g.* FROM groups g
        INNER JOIN user_group ug ON g.g_id = ug.group_id
        WHERE ug.user_id = ?
        "#,
    )
        .bind(user_id)
        .fetch_all(pool)
        .await
}
