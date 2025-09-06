use axum::Router;
use sqlx::SqlitePool;

mod auth_routes;
mod message_routes;
mod invitation_routes;
mod group_routes;
mod user_routes;

pub fn build_router(db_pool: SqlitePool) -> Router {
    Router::new() 
        .nest("/auth", auth_routes::auth_routes())
        .nest("/message", message_routes::message_routes())
        .nest("/invitations", invitation_routes::invitation_routes())
        .nest("/groups", group_routes::group_routes())
        .nest("/users", user_routes::user_routes())
        .with_state(db_pool)
}
