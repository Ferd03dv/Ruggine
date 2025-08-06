use sqlx::{SqlitePool, query, query_as};
use crate::models::group::Group;
use crate::models::group::UserGroup;

// POST: Crea un gruppo
pub async fn create_group(pool: &SqlitePool, name_group: &str, created_by: i64) -> Result<(), sqlx::Error> {
    query("INSERT INTO groups (name_group, created_by) VALUES (?, ?)")
        .bind(name_group)
        .bind(created_by)
        .execute(pool)
        .await?;
    Ok(())
}

// POST: Inserimento in User_Group -> NON MI CONVINCE, MA PER ORA LA LASCIO!
// DOVREI PASSARE da invitation???
pub async fn invite_user_to_group(
    pool: &SqlitePool,
    user_id: i64,
    group_id: i64,
    is_admin: bool,
) -> Result<(), sqlx::Error> {
    query("INSERT INTO user_group (user_id, group_id, is_admin) VALUES (?, ?, ?)")
        .bind(user_id)
        .bind(group_id)
        .bind(is_admin)
        .execute(pool)
        .await?;
    Ok(())
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
