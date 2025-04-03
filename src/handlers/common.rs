use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct PaginationResponse<T> {
    pub data: T,
    pub total: u64,
    pub current_page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}
