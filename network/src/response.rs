use serde::Deserialize;
use std::collections::HashMap;

/// HTTP response
#[derive(Debug, Clone)]
pub struct Response {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    /// Create a new response
    pub fn new(status_code: u16, headers: HashMap<String, String>, body: Vec<u8>) -> Self {
        Self {
            status_code,
            headers,
            body,
        }
    }

    /// Get the response body as a string
    pub fn text(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.body.clone())
    }

    /// Deserialize the response body as JSON
    pub fn json<T: for<'de> Deserialize<'de>>(&self) -> crate::Result<T> {
        let value: T = serde_json::from_slice(&self.body)?;
        Ok(value)
    }

    /// Check if the response status is successful (2xx)
    pub fn is_success(&self) -> bool {
        self.status_code >= 200 && self.status_code < 300
    }

    /// Check if the response status is a client error (4xx)
    pub fn is_client_error(&self) -> bool {
        self.status_code >= 400 && self.status_code < 500
    }

    /// Check if the response status is a server error (5xx)
    pub fn is_server_error(&self) -> bool {
        self.status_code >= 500 && self.status_code < 600
    }
}

