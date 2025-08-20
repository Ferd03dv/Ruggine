use sqlx::SqlitePool;
use crate::models::Message;
use crate::repositories::message_repository;

pub struct MessageService {
    pool: SqlitePool,
}

impl MessageService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn send_message(
        &self,
        group_id: i64,
        sender_id: i64,
        content: &str,
    ) -> Result<Message, sqlx::Error> {
        // First insert the message and get the ID
        let message_id = message_repository::send_message(&self.pool, group_id, sender_id, content).await?;
        
        // Then retrieve the specific message by ID
        let message = sqlx::query_as::<_, Message>("SELECT * FROM message WHERE m_id = ?")
            .bind(message_id)
            .fetch_one(&self.pool)
            .await?;
        
        Ok(message)
    }

    pub async fn get_messages_by_group(&self, group_id: i64) -> Result<Vec<Message>, sqlx::Error> {
        message_repository::get_messages_by_group(&self.pool, group_id).await
    }
}
