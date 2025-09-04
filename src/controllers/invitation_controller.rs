use axum::{
    extract::{State, Path}, 
    http::StatusCode,
    Json};
use sqlx::SqlitePool;
use serde::Deserialize;
use crate::services::invitation_service::InvitationService;

#[derive(Deserialize)]
pub struct CreateInvitation {
    i_id: Option<i64>,
    status: i64,
    send_at: chrono::NaiveDateTime,
    invited_by: i64,
    invited_user: i64,
    group_id: i64,
}

pub async fn create_invitation(
    State(db_pool): State<SqlitePool>,
    Json(payload): Json<CreateInvitation>,
) -> Result<StatusCode, (StatusCode, String)> {
    let service = InvitationService::new(db_pool.clone());
    service.create_invitation(payload.i_id, payload.status, payload.send_at, payload.invited_by, payload.invited_user, payload.group_id)
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
