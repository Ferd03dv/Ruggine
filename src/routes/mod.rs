use axum::Router;
use sqlx::SqlitePool;

mod auth_routes; // etc.

pub fn build_router(db_pool: SqlitePool) -> Router {
    Router::new() 
    // Andrò poi a creare le vere Route nei vari file: auth_routes, user_routes, ...
    // .merge(auth_routes::router(db_pool.clone()))
    // .merge(altri router)
}
