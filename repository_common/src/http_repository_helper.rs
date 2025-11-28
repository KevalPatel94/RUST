use network::{HTTPClient, HTTPClientImpl, Response, NetworkError};
use serde::de::DeserializeOwned;
use thiserror::Error;

/// Common error for repository operations
#[derive(Debug, Error)]
pub enum RepositoryCommonError {
    #[error("Network error: {0}")]
    Network(#[from] NetworkError),
    #[error("JSON parsing error: {0}")]
    JsonParse(#[from] serde_json::Error),
    #[error("HTTP error: status {0}")]
    HttpError(u16),
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

/// Helper struct for common HTTP repository operations
/// This provides reusable HTTP request/response handling that can be used
/// across multiple repository crates (user_data, product_data, etc.)
pub struct HttpRepositoryHelper {
    client: HTTPClientImpl,
    base_url: String,
}

impl HttpRepositoryHelper {
    /// Create a new helper with HTTP client and base URL
    pub fn new(client: HTTPClientImpl, base_url: String) -> Self {
        Self { client, base_url }
    }

    /// Generic GET request with automatic JSON parsing
    pub async fn get<T: DeserializeOwned>(
        &self,
        endpoint: &str,
    ) -> Result<T, RepositoryCommonError> {
        let url = format!("{}/{}", self.base_url, endpoint.trim_start_matches('/'));
        
        let response = self
            .client
            .get(url)
            .await
            .map_err(RepositoryCommonError::Network)?;

        Self::parse_response(response).await
    }

    /// Generic GET request with query parameters
    pub async fn get_with_query<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        query_params: &[(String, String)],
    ) -> Result<T, RepositoryCommonError> {
        let mut url = format!("{}/{}", self.base_url, endpoint.trim_start_matches('/'));
        
        if !query_params.is_empty() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&");
            url = format!("{}?{}", url, query_string);
        }

        let response = self
            .client
            .get(url)
            .await
            .map_err(RepositoryCommonError::Network)?;

        Self::parse_response(response).await
    }

    /// Generic POST request
    pub async fn post<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &[u8],
    ) -> Result<T, RepositoryCommonError> {
        let url = format!("{}/{}", self.base_url, endpoint.trim_start_matches('/'));
        
        let response = self
            .client
            .post(url, body.to_vec())
            .await
            .map_err(RepositoryCommonError::Network)?;

        Self::parse_response(response).await
    }

    /// Parse HTTP response with common error handling
    /// This centralizes response parsing logic that all repositories need
    pub async fn parse_response<T: DeserializeOwned>(
        response: Response,
    ) -> Result<T, RepositoryCommonError> {
        if !(200..300).contains(&response.status_code) {
            return Err(RepositoryCommonError::HttpError(response.status_code));
        }

        let body_str = String::from_utf8_lossy(&response.body);
        serde_json::from_str(&body_str)
            .map_err(RepositoryCommonError::JsonParse)
    }

    /// GET request with retry logic and exponential backoff
    pub async fn get_with_retry<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        max_retries: u32,
    ) -> Result<T, RepositoryCommonError> {
        let mut last_error = None;
        
        for attempt in 0..=max_retries {
            match self.get(endpoint).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    last_error = Some(e);
                    if attempt < max_retries {
                        // Exponential backoff: 100ms, 200ms, 400ms, etc.
                        let delay_ms = 100 * 2_u64.pow(attempt);
                        tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
                    }
                }
            }
        }
        
        Err(last_error.unwrap())
    }
}

