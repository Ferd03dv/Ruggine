mod config {
    pub mod database;
}
mod models;
mod repositories;
mod routes;
mod server;
mod controllers;
mod services;
mod utils;

use utils::cpu_logger::CpuLogger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cpu_logger = CpuLogger::new();
    cpu_logger.start_logging().await;
    
    server::run().await
}
