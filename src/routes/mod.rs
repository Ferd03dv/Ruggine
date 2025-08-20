use axum::Router;
use sqlx::SqlitePool;

mod auth_routes;
mod message_routes;

pub fn build_router(db_pool: SqlitePool) -> Router {
    Router::new() 
        .nest("/auth", auth_routes::auth_routes())
        .nest("/", message_routes::message_routes())
        .with_state(db_pool)
}
