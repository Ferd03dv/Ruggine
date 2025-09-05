use axum::{
    extract::{State, Path}, 
    http::StatusCode,
    Json};
use sqlx::SqlitePool;
use serde::{Deserialize, Serialize};
use crate::services::invitation_service::InvitationService;
use crate::models::Invitation;

#[derive(Deserialize)]
pub struct CreateInvitation {
    i_id: Option<i64>,
    status: i64,
    sent_at: chrono::NaiveDateTime,
    invited_by: i64,
    invited_user: i64,
    group_id: i64,
}

#[derive(Serialize)]
pub struct InvitationListResponse {
    pub invitations: Vec<Invitation>,
}

pub async fn create_invitation(
    State(db_pool): State<SqlitePool>,
    Json(payload): Json<CreateInvitation>,
) -> Result<StatusCode, (StatusCode, String)> {
    let service = InvitationService::new(db_pool.clone());
    service.create_invitation(payload.i_id, payload.status, payload.sent_at, payload.invited_by, payload.invited_user, payload.group_id)
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

pub async fn accept_invitation(
    State(db_pool): State<SqlitePool>,
    Path(invitation_id): Path<i64>,
) -> Result<StatusCode, (StatusCode, String)> {
    let service = InvitationService::new(db_pool.clone());
    service.accept_invitation(invitation_id)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

pub async fn reject_invitation(
    State(db_pool): State<SqlitePool>,
    Path(invitation_id): Path<i64>,
) -> Result<StatusCode, (StatusCode, String)> {
    let service = InvitationService::new(db_pool.clone());
    service.reject_invitation(invitation_id)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

pub async fn get_user_invitations(
    State(db_pool): State<SqlitePool>,
    Path(user_id): Path<i64>,
) -> Result<Json<InvitationListResponse>, (StatusCode, String)> {
    let service = InvitationService::new(db_pool.clone());
    match service.get_user_invitations(user_id).await {
        Ok(invitations) => Ok(Json(InvitationListResponse { invitations })),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}
