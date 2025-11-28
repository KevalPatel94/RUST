use serde::{Deserialize, Serialize};

/// Pagination parameters for API requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    pub page: u32,
    pub limit: u32,
}

impl PaginationParams {
    /// Create new pagination parameters
    pub fn new(page: u32, limit: u32) -> Self {
        Self { page, limit }
    }

    /// Default pagination (page 1, limit 20)
    pub fn default() -> Self {
        Self { page: 1, limit: 20 }
    }

    /// Convert to query parameters for HTTP requests
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        vec![
            ("page".to_string(), self.page.to_string()),
            ("limit".to_string(), self.limit.to_string()),
        ]
    }
}

/// Paginated response wrapper for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: u32,
    pub page: u32,
    pub limit: u32,
    pub has_more: bool,
}

impl<T> PaginatedResponse<T> {
    /// Create a new paginated response
    pub fn new(data: Vec<T>, total: u32, page: u32, limit: u32) -> Self {
        Self {
            has_more: (page * limit) < total,
            data,
            total,
            page,
            limit,
        }
    }
}

