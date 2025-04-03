use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct PaginationQuery {
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug, Serialize)]
pub struct PaginationResponse<T> {
    pub data: T,
    pub total: u64,
    pub current_page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}
