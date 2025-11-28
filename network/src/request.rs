use serde::Serialize;
use std::collections::HashMap;

/// HTTP methods
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Delete => "DELETE",
        }
    }
}

/// HTTP request configuration
#[derive(Debug, Clone)]
pub struct Request {
    pub method: HttpMethod,
    pub url: String,
    pub headers: Option<HashMap<String, String>>,
    pub query_params: Option<HashMap<String, String>>,
    pub body: Option<Vec<u8>>,
    pub timeout_seconds: Option<u64>,
}

impl Request {
    /// Create a new GET request
    pub fn get(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Get,
            url: url.into(),
            headers: None,
            query_params: None,
            body: None,
            timeout_seconds: None,
        }
    }

    /// Create a new POST request
    pub fn post(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Post,
            url: url.into(),
            headers: None,
            query_params: None,
            body: None,
            timeout_seconds: None,
        }
    }

    /// Create a new PUT request
    pub fn put(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Put,
            url: url.into(),
            headers: None,
            query_params: None,
            body: None,
            timeout_seconds: None,
        }
    }

    /// Create a new PATCH request
    pub fn patch(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Patch,
            url: url.into(),
            headers: None,
            query_params: None,
            body: None,
            timeout_seconds: None,
        }
    }

    /// Create a new DELETE request
    pub fn delete(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Delete,
            url: url.into(),
            headers: None,
            query_params: None,
            body: None,
            timeout_seconds: None,
        }
    }

    /// Add a header to the request
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        if self.headers.is_none() {
            self.headers = Some(HashMap::new());
        }
        self.headers
            .as_mut()
            .unwrap()
            .insert(key.into(), value.into());
        self
    }

    /// Add multiple headers to the request
    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        if self.headers.is_none() {
            self.headers = Some(HashMap::new());
        }
        self.headers.as_mut().unwrap().extend(headers);
        self
    }

    /// Add a query parameter
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        if self.query_params.is_none() {
            self.query_params = Some(HashMap::new());
        }
        self.query_params
            .as_mut()
            .unwrap()
            .insert(key.into(), value.into());
        self
    }

    /// Add multiple query parameters
    pub fn query_params(mut self, params: HashMap<String, String>) -> Self {
        if self.query_params.is_none() {
            self.query_params = Some(HashMap::new());
        }
        self.query_params.as_mut().unwrap().extend(params);
        self
    }

    /// Set the request body as raw bytes
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    /// Set the request body as JSON
    /// Automatically sets Content-Type header to application/json
    pub fn json<T: Serialize>(mut self, data: &T) -> crate::Result<Self> {
        let json = serde_json::to_vec(data)?;
        self.body = Some(json);
        
        // Automatically set Content-Type header if not already set
        if self.headers.is_none() {
            self.headers = Some(HashMap::new());
        }
        let headers = self.headers.as_mut().unwrap();
        if !headers.contains_key("Content-Type") && !headers.contains_key("content-type") {
            headers.insert("Content-Type".to_string(), "application/json".to_string());
        }
        
        Ok(self)
    }

    /// Set the request timeout in seconds
    pub fn timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = Some(seconds);
        self
    }
}

