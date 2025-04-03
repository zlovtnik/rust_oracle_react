use crate::handlers::common::{ErrorResponse, PaginationResponse};
use crate::models::nfe_identification::{CreateNFeIdentification, NFeIdentification};
use crate::repositories::nfe_identification_repository::NFeIdentificationRepository;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::web::{self, Query};
use actix_web::{delete, get, post, put, HttpResponse, Responder};
use serde::Deserialize;
use std::sync::Arc;
use tracing::{error, info, instrument};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(list_identifications)
            .service(get_identification)
            .service(create_identification)
            .service(update_identification)
            .service(delete_identification),
    );
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
    #[serde(default)]
    pub nat_op: Option<String>,
    #[serde(default)]
    pub n_nf: Option<String>,
    #[serde(default)]
    pub tp_nf: Option<String>,
    #[serde(default)]
    pub dh_emi: Option<String>,
    #[serde(default)]
    pub search: Option<String>,
}

fn default_page() -> u32 {
    1
}

fn default_page_size() -> u32 {
    50
}

#[get("/nfe-identifications")]
#[instrument(skip(repo))]
pub async fn list_identifications(
    repo: web::Data<Arc<NFeIdentificationRepository>>,
    query: Query<ListQuery>,
) -> impl Responder {
    info!(
        "Listing NFe identifications with pagination - page: {}, page_size: {}",
        query.page, query.page_size
    );

    match repo
        .find_all(
            query.page,
            query.page_size,
            query.nat_op.clone(),
            query.n_nf.clone(),
            query.tp_nf.clone(),
            query.dh_emi.clone(),
            query.search.clone(),
        )
        .await
    {
        Ok((identifications, total_count)) => {
            let total_pages = (total_count as f64 / query.page_size as f64).ceil() as u32;
            HttpResponse::Ok().json(PaginationResponse {
                data: identifications,
                total: total_count,
                current_page: query.page,
                page_size: query.page_size,
                total_pages,
            })
        }
        Err(e) => {
            error!("Failed to list NFe identifications: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: e.to_string(),
            })
        }
    }
}

#[get("/api/identifications/{id}")]
pub async fn get_identification(
    repo: web::Data<Arc<NFeIdentificationRepository>>,
    id: web::Path<String>,
) -> impl Responder {
    match repo.find_by_id(&id).await {
        Ok(Some(identification)) => HttpResponse::Ok().json(identification),
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Identification not found".to_string(),
        }),
        Err(e) => {
            error!("Failed to get identification: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to get identification".to_string(),
            })
        }
    }
}

#[post("/api/identifications")]
pub async fn create_identification(
    repo: web::Data<Arc<NFeIdentificationRepository>>,
    identification: web::Json<CreateNFeIdentification>,
) -> impl Responder {
    match repo.create(&identification).await {
        Ok(created) => HttpResponse::Created().json(created),
        Err(e) => {
            error!("Failed to create identification: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to create identification".to_string(),
            })
        }
    }
}

#[put("/api/identifications/{id}")]
pub async fn update_identification(
    repo: web::Data<Arc<NFeIdentificationRepository>>,
    id: web::Path<String>,
    identification: web::Json<NFeIdentification>,
) -> impl Responder {
    match repo.update(&id, &identification).await {
        Ok(updated) => HttpResponse::Ok().json(updated),
        Err(e) => {
            error!("Failed to update identification: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to update identification".to_string(),
            })
        }
    }
}

#[delete("/api/identifications/{id}")]
pub async fn delete_identification(
    repo: web::Data<Arc<NFeIdentificationRepository>>,
    id: web::Path<String>,
) -> impl Responder {
    match repo.delete(&id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            error!("Failed to delete identification: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to delete identification".to_string(),
            })
        }
    }
}
