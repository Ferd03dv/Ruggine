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
        if invited_by == invited_user {
            return Err(sqlx::Error::Protocol("Non puoi invitare te stesso".into()));
        }

        let existing = sqlx::query(
            "SELECT i_id FROM invitation WHERE invited_user = ? AND group_id = ? AND (status = 0)"
        )
        .bind(invited_user)
        .bind(group_id)
        .fetch_optional(pool)
        .await?;
        if existing.is_some() {
            return Err(sqlx::Error::Protocol("Invito già presente".into()));
        }

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
        // Aggiorna lo stato dell'invito
        sqlx::query(
            "UPDATE invitation SET status = 1 WHERE i_id = ?"
        )
        .bind(invitation_id)
        .execute(pool)
        .await?;

        // Recupera i dati dell'invito
        let row = sqlx::query("SELECT invited_user, group_id FROM invitation WHERE i_id = ?")
            .bind(invitation_id)
            .fetch_one(pool)
            .await?;
        let invited_user: i64 = row.get("invited_user");
        let group_id: i64 = row.get("group_id");

        // Inserisci la relazione in user_group con is_admin = 0
        sqlx::query("INSERT INTO user_group (user_id, group_id, is_admin) VALUES (?, ?, 0)")
            .bind(invited_user)
            .bind(group_id)
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

    /// Ottieni tutti gli inviti pending di un utente
    pub async fn get_user_invitations(
        pool: &SqlitePool,
        user_id: i64,
    ) -> Result<Vec<Invitation>, sqlx::Error> {
        let invitations = sqlx::query_as::<_, Invitation>(
            "SELECT i_id, status, sent_at, invited_by, invited_user, group_id 
             FROM invitation 
             WHERE invited_user = ? AND status = 0"
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;
        
        Ok(invitations)
    }
}
