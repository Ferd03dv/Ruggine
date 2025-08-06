use sqlx::FromRow;
use chrono::NaiveDateTime;
// NaiveDateTime -> Gestione della data
// Per l'ora di invio del messaggio
// Magari lo si toglie

#[derive(Debug, FromRow)]
pub struct Message {
    pub m_id: i64,
    pub content: String,
    pub sent_at: NaiveDateTime,
    pub sender_id: i64,
    pub group_id: i64,
}
