use axum::{
    routing::{post},
    Router,
};
use sqlx::SqlitePool;
use crate::controllers::invitation_controller::{create_invitation, accept_invitation, reject_invitation};

pub fn invitation_routes() -> Router<SqlitePool> {
    Router::new()
        .route("/invitation/create", post(create_invitation))
        .route("/invitation/:id/accept", post(accept_invitation))
        .route("/invitation/:id/reject", post(reject_invitation))
}