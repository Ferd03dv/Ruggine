use axum::{
    routing::{post, get},
    Router,
};
use sqlx::SqlitePool;
use crate::controllers::group_controller::{create_group, list_groups_by_user};

pub fn group_routes() -> Router<SqlitePool> {
    Router::new()
        .route("/create", post(create_group))
        .route("/user/:user_id", get(list_groups_by_user))
}