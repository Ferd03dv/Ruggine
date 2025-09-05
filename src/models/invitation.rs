use sqlx::FromRow;
use chrono::NaiveDateTime;
use serde::Serialize;
// NaiveDateTime -> Gestione della data
// Per l'ora di invio del messaggio
// Magari lo si toglie

#[derive(Debug, FromRow, Serialize)]
pub struct Invitation {
    pub i_id: Option<i64>,
    pub status: i64, // Nelle query lo gestisco come enum!
    pub sent_at: NaiveDateTime,
    pub invited_by: i64,
    pub invited_user: i64,
    pub group_id: i64,
}
