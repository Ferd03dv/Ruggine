use axum::{
    routing::get,
    Router,
};
use sqlx::SqlitePool;
use crate::controllers::user_controller;

pub fn user_routes() -> Router<SqlitePool> {
    Router::new()
        .route("/user/:id", get(user_controller::get_user_by_id))
}
