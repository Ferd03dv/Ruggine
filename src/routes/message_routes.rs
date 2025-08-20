use axum::{
    routing::{get, post},
    Router,
};
use sqlx::SqlitePool;
use crate::controllers::message_controller::{send_message, get_messages};

pub fn message_routes() -> Router<SqlitePool> {
    Router::new()
        .route("/groups/:id/message", post(send_message))
        .route("/groups/:id/messages", get(get_messages))
}
