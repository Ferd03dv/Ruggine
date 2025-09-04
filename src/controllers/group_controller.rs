use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;
use serde::{Deserialize, Serialize};
use crate::services::group_service::GroupService;
use crate::models::group::Group;

#[derive(Deserialize)]
pub struct CreateGroup {
    pub name: String,
    pub created_by: i64,
}

#[derive(Serialize)]
pub struct GroupListResponse {
    pub groups: Vec<Group>,
}

pub async fn create_group(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateGroup>,
) -> Result<StatusCode, (StatusCode, String)> {
    GroupService::create_group(&pool, &payload.name, payload.created_by)
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

pub async fn list_groups_by_user(
    State(pool): State<SqlitePool>,
    Path(user_id): Path<i64>,
) -> Result<Json<GroupListResponse>, (StatusCode, String)> {
    match GroupService::list_groups_by_user(&pool, user_id).await {
        Ok(groups) => Ok(Json(GroupListResponse { groups })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}