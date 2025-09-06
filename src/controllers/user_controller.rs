use axum::extract::{State, Path};
use axum::http::StatusCode;
use axum::response::Json;
use serde::Serialize;
use sqlx::SqlitePool;
use crate::models::User;
use crate::repositories::user_repository;

#[derive(Serialize)]
pub struct UserResponse {
    pub success: bool,
    pub user: Option<User>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub message: String,
}

// GET /user/:id
pub async fn get_user_by_id(
    State(pool): State<SqlitePool>,
    Path(user_id): Path<i64>,
) -> Result<Json<UserResponse>, (StatusCode, Json<ErrorResponse>)> {
    match user_repository::get_user_by_id(&pool, user_id).await {
        Ok(Some(user)) => Ok(Json(UserResponse {
            success: true,
            user: Some(user),
        })),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                success: false,
                message: "User not found".to_string(),
            }),
        )),
        Err(e) => {
            eprintln!("Database error during user lookup: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    success: false,
                    message: "Internal server error".to_string(),
                }),
            ))
        }
    }
}
