use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use oracle::Connection;
use redis::aio::ConnectionManager;
use redis::Client;
use std::env;
use std::sync::Arc;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

mod handlers;
mod models;
mod repositories;
mod services;

use handlers::nfe_identification_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize tracing
    let _subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_names(true)
        .with_ansi(true)
        .with_level(true)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .init();

    // Get environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    // Parse the database URL into username, password, and connect string
    let parts: Vec<&str> = database_url.split("://").collect();
    if parts.len() != 2 {
        error!("Invalid DATABASE_URL format");
        panic!("Invalid DATABASE_URL format");
    }

    let credentials: Vec<&str> = parts[1].split('@').collect();
    if credentials.len() != 2 {
        error!("Invalid DATABASE_URL format");
        panic!("Invalid DATABASE_URL format");
    }

    let user_pass: Vec<&str> = credentials[0].split(':').collect();
    if user_pass.len() != 2 {
        error!("Invalid DATABASE_URL format");
        panic!("Invalid DATABASE_URL format");
    }

    let username = user_pass[0];
    let password = user_pass[1];
    let connect_string = credentials[1];

    info!("Attempting to connect to Oracle database...");
    let oracle_conn = match Connection::connect(username, password, connect_string) {
        Ok(conn) => {
            info!("Successfully connected to Oracle database");
            conn
        }
        Err(e) => {
            error!("Failed to connect to Oracle database: {}", e);
            panic!("Failed to connect to Oracle database: {}", e);
        }
    };

    // Connect to Redis
    let redis_client = Client::open(redis_url).expect("Failed to create Redis client");
    let redis_manager = ConnectionManager::new(redis_client)
        .await
        .expect("Failed to create Redis connection manager");

    info!("Successfully connected to both Oracle and Redis");

    // Create repository
    let nfe_repo = Arc::new(
        repositories::nfe_identification_repository::NFeIdentificationRepository::new(
            oracle_conn,
            redis_manager,
        ),
    );

    info!("Starting HTTP server on 0.0.0.0:{}", port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&nfe_repo)))
            .configure(nfe_identification_handler::init_routes)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
