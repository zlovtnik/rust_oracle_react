use actix_web::{web, App, Error as ActixError, HttpServer, Result as ActixResult};
use dotenv::dotenv;
use oracle::Connection;
use std::env;
use std::sync::Arc;

mod handlers;
mod models;
mod repositories;

use handlers::nfe_identification_handler::*;
use repositories::nfe_identification_repository::NFeIdentificationRepository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Parse the database URL into username, password, and connect string
    let parts: Vec<&str> = database_url.split("://").collect();
    if parts.len() != 2 {
        panic!("Invalid DATABASE_URL format");
    }

    let credentials: Vec<&str> = parts[1].split('@').collect();
    if credentials.len() != 2 {
        panic!("Invalid DATABASE_URL format");
    }

    let user_pass: Vec<&str> = credentials[0].split(':').collect();
    if user_pass.len() != 2 {
        panic!("Invalid DATABASE_URL format");
    }

    let username = user_pass[0];
    let password = user_pass[1];
    let connect_string = credentials[1];

    let conn = Connection::connect(username, password, connect_string)
        .expect("Failed to connect to Oracle database");

    let nfe_repo = Arc::new(NFeIdentificationRepository::new(conn));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&nfe_repo)))
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/identifications")
                            .route(web::get().to(list_identifications))
                            .route(web::post().to(create_identification)),
                    )
                    .service(
                        web::resource("/identifications/{id}")
                            .route(web::get().to(get_identification))
                            .route(web::put().to(update_identification))
                            .route(web::delete().to(delete_identification)),
                    ),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
