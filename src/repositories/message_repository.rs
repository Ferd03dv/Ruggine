use sqlx::SqlitePool;
use crate::models::Message;

// POST: Crea un nuovo messaggio: conterrà ID di gruppo, utente che lo invia e il proprio autogenerato
pub async fn send_message(
    pool: &SqlitePool,
    group_id: i64,
    sender_id: i64,
    content: &str,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO message (group_id, sender_id, content, sent_at) VALUES (?, ?, ?, datetime('now'))")
        .bind(group_id)
        .bind(sender_id)
        .bind(content)
        .execute(pool)
        .await?;
    Ok(result.last_insert_rowid())
}

// GET: Ottengo tutti i messaggi di un certo gruppo (in base all'id del gruppo che riceve come parametro)
pub async fn get_messages_by_group(pool: &SqlitePool, group_id: i64) -> Result<Vec<Message>, sqlx::Error> {
    sqlx::query_as::<_, Message>("SELECT * FROM message WHERE group_id = ? ORDER BY sent_at DESC")
        .bind(group_id)
        .fetch_all(pool)
        .await
}

