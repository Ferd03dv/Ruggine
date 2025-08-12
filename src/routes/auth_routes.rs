use axum::{
    routing::post,
    Router,
};
use sqlx::SqlitePool;
use crate::controllers::auth_controller;

pub fn auth_routes() -> Router<SqlitePool> {
    Router::new()
        .route("/login", post(auth_controller::login))
        .route("/register", post(auth_controller::register))
}