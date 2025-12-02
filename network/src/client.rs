use crate::error::{NetworkError, Result};
use crate::request::Request;
use crate::response::Response;
use reqwest::Client as ReqwestClient;
use std::collections::HashMap;
use std::time::Duration;
use url::Url;

/// Trait for HTTP client operations
/// This trait allows for dependency injection and testing
#[async_trait::async_trait]
pub trait HTTPClient: Send + Sync {
    /// Execute an HTTP request
    async fn execute(&self, request: Request) -> Result<Response>;

    /// Execute a GET request
    async fn get(&self, url: impl Into<String> + Send) -> Result<Response> {
        self.execute(Request::get(url)).await
    }

    /// Execute a POST request with JSON body
    async fn post_json<T: serde::Serialize + Sync>(
        &self,
        url: impl Into<String> + Send,
        data: &T,
    ) -> Result<Response> {
        let request = Request::post(url).json(data)?;
        self.execute(request).await
    }

    /// Execute a POST request with raw body
    async fn post(&self, url: impl Into<String> + Send, body: Vec<u8>) -> Result<Response> {
        let request = Request::post(url).body(body);
        self.execute(request).await
    }

    /// Execute a PUT request with JSON body
    async fn put_json<T: serde::Serialize + Sync>(
        &self,
        url: impl Into<String> + Send,
        data: &T,
    ) -> Result<Response> {
        let request = Request::put(url).json(data)?;
        self.execute(request).await
    }

    /// Execute a PUT request with raw body
    async fn put(&self, url: impl Into<String> + Send, body: Vec<u8>) -> Result<Response> {
        let request = Request::put(url).body(body);
        self.execute(request).await
    }

    /// Execute a PATCH request with JSON body
    async fn patch_json<T: serde::Serialize + Sync>(
        &self,
        url: impl Into<String> + Send,
        data: &T,
    ) -> Result<Response> {
        let request = Request::patch(url).json(data)?;
        self.execute(request).await
    }

    /// Execute a DELETE request
    async fn delete(&self, url: impl Into<String> + Send) -> Result<Response> {
        self.execute(Request::delete(url)).await
    }
}

/// Concrete implementation of HTTPClient using reqwest
pub struct HTTPClientImpl {
    client: ReqwestClient,
    default_timeout: Option<Duration>,
    default_headers: Option<HashMap<String, String>>,
}

impl HTTPClientImpl {
    /// Create a new HTTPClientImpl with default configuration
    /// Configured for cross-platform compatibility (Android/iOS/Web)
    pub fn new() -> Result<Self> {
        let client = ReqwestClient::builder()
            // Set reasonable timeout for mobile networks
            .timeout(Duration::from_secs(30))
            // Enable connection pooling for better performance
            .pool_idle_timeout(Duration::from_secs(90))
            // Set user agent for better compatibility
            .user_agent("LockSmith/1.0")
            .build()
            .map_err(|e| NetworkError::ConnectionError(e.to_string()))?;

        Ok(Self {
            client,
            default_timeout: Some(Duration::from_secs(30)),
            default_headers: None,
        })
    }

    /// Create a new HTTPClientImpl with a custom timeout
    pub fn with_timeout(timeout_seconds: u64) -> Result<Self> {
        let client = ReqwestClient::builder()
            .timeout(Duration::from_secs(timeout_seconds))
            .build()
            .map_err(|e| NetworkError::ConnectionError(e.to_string()))?;

        Ok(Self {
            client,
            default_timeout: Some(Duration::from_secs(timeout_seconds)),
            default_headers: None,
        })
    }

    /// Create a new HTTPClientImpl with default headers
    pub fn with_headers(headers: HashMap<String, String>) -> Result<Self> {
        let client = ReqwestClient::builder()
            .build()
            .map_err(|e| NetworkError::ConnectionError(e.to_string()))?;

        Ok(Self {
            client,
            default_timeout: None,
            default_headers: Some(headers),
        })
    }

    /// Create a new HTTPClientImpl with timeout and default headers
    pub fn with_config(timeout_seconds: Option<u64>, headers: Option<HashMap<String, String>>) -> Result<Self> {
        let mut client_builder = ReqwestClient::builder();

        if let Some(timeout) = timeout_seconds {
            client_builder = client_builder.timeout(Duration::from_secs(timeout));
        }

        let client = client_builder
            .build()
            .map_err(|e| NetworkError::ConnectionError(e.to_string()))?;

        Ok(Self {
            client,
            default_timeout: timeout_seconds.map(Duration::from_secs),
            default_headers: headers,
        })
    }
}

impl Default for HTTPClientImpl {
    fn default() -> Self {
        Self::new().expect("Failed to create default HTTPClientImpl")
    }
}

#[async_trait::async_trait]
impl HTTPClient for HTTPClientImpl {
    async fn execute(&self, request: Request) -> Result<Response> {
        // Parse URL
        let mut url = Url::parse(&request.url)?;

        // Add query parameters
        if let Some(ref query_params) = request.query_params {
            let mut query_pairs = url.query_pairs_mut();
            for (key, value) in query_params {
                query_pairs.append_pair(key, value);
            }
        }

        // Build the request
        let mut http_request = match request.method {
            crate::request::HttpMethod::Get => self.client.get(url.as_str()),
            crate::request::HttpMethod::Post => self.client.post(url.as_str()),
            crate::request::HttpMethod::Put => self.client.put(url.as_str()),
            crate::request::HttpMethod::Patch => self.client.patch(url.as_str()),
            crate::request::HttpMethod::Delete => self.client.delete(url.as_str()),
        };

        // Add default headers first (if not already in request headers)
        if let Some(ref default_headers) = self.default_headers {
            for (key, value) in default_headers {
                // Only add if not already present in request headers
                let already_present = request.headers
                    .as_ref()
                    .map(|h| h.contains_key(key))
                    .unwrap_or(false);
                if !already_present {
                    http_request = http_request.header(key, value);
                }
            }
        }

        // Add request-specific headers (these will override defaults)
        if let Some(ref headers) = request.headers {
            for (key, value) in headers {
                http_request = http_request.header(key, value);
            }
        }

        // Set timeout if specified
        if let Some(timeout) = request.timeout_seconds {
            http_request = http_request.timeout(Duration::from_secs(timeout));
        } else if let Some(default_timeout) = self.default_timeout {
            http_request = http_request.timeout(default_timeout);
        }

        // Add body
        if let Some(body) = request.body {
            http_request = http_request.body(body);
        }

        // Execute the request
        let response = http_request.send().await?;

        // Extract status code
        let status_code = response.status().as_u16();

        // Extract headers
        let mut headers = HashMap::new();
        for (key, value) in response.headers() {
            if let Ok(value_str) = value.to_str() {
                headers.insert(key.to_string(), value_str.to_string());
            }
        }

        // Extract body
        let body = response.bytes().await?.to_vec();

        Ok(Response::new(status_code, headers, body))
    }
}

