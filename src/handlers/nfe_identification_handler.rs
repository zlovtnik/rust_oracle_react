use crate::models::nfe_identification::{CreateNFeIdentification, NFeIdentification};
use crate::repositories::nfe_identification_repository::{
    NFeIdentificationRepository, RepositoryError,
};
use actix_web::{web, HttpResponse, Responder};
use serde_json;
use std::sync::Arc;
use uuid::Uuid;

pub async fn list_identifications(
    repo: web::Data<Arc<NFeIdentificationRepository>>,
) -> impl Responder {
    match repo.find_all().await {
        Ok(identifications) => HttpResponse::Ok().json(identifications),
        Err(e) => {
            eprintln!("Error fetching identifications: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to fetch identifications")
        }
    }
}

pub async fn get_identification(
    repo: web::Data<Arc<NFeIdentificationRepository>>,
    path: web::Path<String>,
) -> impl Responder {
    let internal_key = path.into_inner();
    match repo.find_by_id(&internal_key).await {
        Ok(Some(identification)) => HttpResponse::Ok().json(identification),
        Ok(None) => HttpResponse::NotFound().json("Identification not found"),
        Err(e) => {
            eprintln!("Error fetching identification: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to fetch identification")
        }
    }
}

pub async fn create_identification(
    repo: web::Data<Arc<NFeIdentificationRepository>>,
    identification: web::Json<CreateNFeIdentification>,
) -> impl Responder {
    // Log the received data
    println!("Received identification data: {:?}", identification);
    println!(
        "Raw JSON: {}",
        serde_json::to_string_pretty(&identification).unwrap()
    );

    // Validate required fields
    if identification.c_uf.is_empty() {
        println!("cUF field is empty");
        return HttpResponse::BadRequest().json("cUF field is required");
    }

    // Log the parsed data
    println!("Parsed cUF: {}", identification.c_uf);
    println!("Parsed cNF: {}", identification.c_nf);
    println!("Parsed dhEmi: {:?}", identification.dh_emi);

    match repo.create(&identification.into_inner()).await {
        Ok(created) => {
            println!("Successfully created identification: {:?}", created);
            HttpResponse::Created().json(created)
        }
        Err(e) => {
            eprintln!("Error creating identification: {:?}", e);
            HttpResponse::BadRequest().json(format!("Failed to create identification: {}", e))
        }
    }
}

pub async fn update_identification(
    repo: web::Data<Arc<NFeIdentificationRepository>>,
    path: web::Path<String>,
    identification: web::Json<NFeIdentification>,
) -> impl Responder {
    let internal_key = path.into_inner();
    println!(
        "Updating identification with internal_key: {}",
        internal_key
    );
    println!("Update data: {:?}", identification);

    match repo
        .update(&internal_key, &identification.into_inner())
        .await
    {
        Ok(updated) => {
            println!("Successfully updated identification: {:?}", updated);
            HttpResponse::Ok().json(updated)
        }
        Err(e) => {
            eprintln!("Error updating identification: {:?}", e);
            match e {
                RepositoryError::NotFound => {
                    HttpResponse::NotFound().json("Identification not found")
                }
                RepositoryError::InvalidUuid(msg) => {
                    HttpResponse::BadRequest().json(format!("Invalid UUID: {}", msg))
                }
                RepositoryError::UpdateFailed => {
                    HttpResponse::InternalServerError().json("Failed to update identification")
                }
                RepositoryError::OracleError(e) => {
                    HttpResponse::InternalServerError().json(format!("Database error: {}", e))
                }
                _ => HttpResponse::InternalServerError().json("An unexpected error occurred"),
            }
        }
    }
}

pub async fn delete_identification(
    repo: web::Data<Arc<NFeIdentificationRepository>>,
    path: web::Path<String>,
) -> impl Responder {
    let internal_key = path.into_inner();
    match repo.delete(&internal_key).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            eprintln!("Error deleting identification: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to delete identification")
        }
    }
}
