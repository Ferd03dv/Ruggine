use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use crate::services::auth_service;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
    pub user_id: Option<i64>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub message: String,
}

// POST /auth/login
pub async fn login(
    State(pool): State<SqlitePool>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ErrorResponse>)> {
    match auth_service::authenticate_user(&pool, &request.username, &request.password).await {
        Ok(Some(user)) => Ok(Json(AuthResponse {
            success: true,
            message: "Login successful".to_string(),
            user_id: Some(user.u_id),
        })),
        Ok(None) => Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                success: false,
                message: "Invalid username or password".to_string(),
            }),
        )),
        Err(e) => {
            eprintln!("Database error during login: {}", e);
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

// POST /auth/register
pub async fn register(
    State(pool): State<SqlitePool>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ErrorResponse>)> {
    match auth_service::register_user(&pool, &request.username, &request.email, &request.password).await {
        Ok(user_id) => Ok(Json(AuthResponse {
            success: true,
            message: "Registration successful".to_string(),
            user_id: Some(user_id),
        })),
        Err(auth_service::AuthError::UserAlreadyExists(field)) => Err((
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                success: false,
                message: format!("{} already exists", field),
            }),
        )),
        Err(auth_service::AuthError::DatabaseError(e)) => {
            eprintln!("Database error during registration: {}", e);
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
