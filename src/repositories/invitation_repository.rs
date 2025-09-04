use sqlx::{SqlitePool, Row};
use crate::models::invitation::Invitation;

#[derive(Debug)]
pub enum InvitationStatus {
    Pending,
    Accepted,
    Rejected,
}

impl ToString for InvitationStatus {
    fn to_string(&self) -> String {
        match self {
            InvitationStatus::Pending => "pending".to_string(),
            InvitationStatus::Accepted => "accepted".to_string(),
            InvitationStatus::Rejected => "rejected".to_string(),
        }
    }
}

pub struct InvitationRepository;

impl InvitationRepository {
    /// Crea un nuovo invito
    pub async fn create_invitation(
        pool: &SqlitePool,
        i_id: Option<i64>,
        status: i64,
        sent_at: chrono::NaiveDateTime,
        invited_by: i64,
        invited_user: i64,
        group_id: i64,
    ) -> Result<i64, sqlx::Error> {
        let rec = sqlx::query(
            "INSERT INTO invitation (status, sent_at, invited_by, invited_user, group_id)
             VALUES (?, ?, ?, ?, ?)"
        )
        .bind(status)      
        .bind(sent_at.format("%Y-%m-%d %H:%M:%S").to_string()) 
        .bind(invited_by)  
        .bind(invited_user) 
        .bind(group_id)     
        .execute(pool)
        .await?;

        Ok(rec.last_insert_rowid())
    }

    /// Accetta un invito
    pub async fn accept_invitation(
        pool: &SqlitePool,
        invitation_id: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE invitation SET status = 1 WHERE i_id = ?"
        )
        .bind(invitation_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Rifiuta un invito
    pub async fn reject_invitation(
        pool: &SqlitePool,
        invitation_id: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE invitation SET status = 2 WHERE i_id = ?"
        )
        .bind(invitation_id)
        .execute(pool)
        .await?;
        Ok(())
    }
}
