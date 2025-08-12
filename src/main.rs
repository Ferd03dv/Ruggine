mod config {
    pub mod database;
}
mod models;
mod repositories;
mod routes;
mod server;
mod controllers;
mod services;

use repositories::user_repository::get_all_users;


use config::database::init_db;
use sqlx::SqlitePool;
use std::{env, fs};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    server::run().await
}
