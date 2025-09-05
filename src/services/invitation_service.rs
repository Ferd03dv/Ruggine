use sqlx::SqlitePool;
use crate::models::{group, Invitation};
use crate::repositories::invitation_repository::InvitationRepository;

pub struct InvitationService {
    pool: SqlitePool,
}

impl InvitationService {
    pub fn new(pool: SqlitePool) -> Self {
        InvitationService { pool }
    }

    pub async fn create_invitation(
        &self,
        i_id: Option<i64>,
        status: i64,
        sent_at: chrono::NaiveDateTime,
        invited_by: i64,
        invited_user: i64,
        group_id: i64,
    ) -> Result<Invitation, sqlx::Error> {
        let invitation_id = InvitationRepository::create_invitation(&self.pool, i_id, status, sent_at, invited_by, invited_user, group_id).await?;

        let invitation = sqlx::query_as::<_, Invitation>("SELECT * FROM invitation WHERE i_id = ?")
            .bind(invitation_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(invitation)
    }

    pub async fn accept_invitation(&self, invitation_id: i64) -> Result<(), sqlx::Error> {
        InvitationRepository::accept_invitation(&self.pool, invitation_id).await
    }

    pub async fn reject_invitation(&self, invitation_id: i64) -> Result<(), sqlx::Error> {
        InvitationRepository::reject_invitation(&self.pool, invitation_id).await
    }

    pub async fn get_user_invitations(&self, user_id: i64) -> Result<Vec<Invitation>, sqlx::Error> {
        InvitationRepository::get_user_invitations(&self.pool, user_id).await
    }
}
