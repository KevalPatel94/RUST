use std::fmt;

/// Result type alias for network operations
pub type Result<T> = std::result::Result<T, NetworkError>;

/// Network error types
#[derive(Debug, Clone)]
pub enum NetworkError {
    /// HTTP request error
    RequestError(String),
    /// HTTP response error
    ResponseError(String),
    /// Network connection error
    ConnectionError(String),
    /// Timeout error
    TimeoutError(String),
    /// JSON serialization/deserialization error
    JsonError(String),
    /// Invalid URL
    InvalidUrl(String),
    /// Invalid request configuration
    InvalidRequest(String),
    /// Unknown error
    Unknown(String),
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkError::RequestError(msg) => write!(f, "Request error: {}", msg),
            NetworkError::ResponseError(msg) => write!(f, "Response error: {}", msg),
            NetworkError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            NetworkError::TimeoutError(msg) => write!(f, "Timeout error: {}", msg),
            NetworkError::JsonError(msg) => write!(f, "JSON error: {}", msg),
            NetworkError::InvalidUrl(msg) => write!(f, "Invalid URL: {}", msg),
            NetworkError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            NetworkError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for NetworkError {}

impl From<reqwest::Error> for NetworkError {
    fn from(err: reqwest::Error) -> Self {
        let error_msg = err.to_string();
        
        // Check for specific error patterns that indicate connection issues
        // "error sending request" is common on mobile platforms when network is unavailable
        if error_msg.contains("error sending request") 
            || error_msg.contains("failed to connect")
            || error_msg.contains("connection refused")
            || error_msg.contains("network unreachable") {
            NetworkError::ConnectionError(error_msg)
        } else if err.is_timeout() {
            NetworkError::TimeoutError(error_msg)
        } else if err.is_connect() {
            NetworkError::ConnectionError(error_msg)
        } else if err.is_request() {
            NetworkError::RequestError(error_msg)
        } else {
            // For unknown errors, check if they're likely connection-related
            if error_msg.to_lowercase().contains("connection") 
                || error_msg.to_lowercase().contains("network")
                || error_msg.to_lowercase().contains("resolve")
                || error_msg.to_lowercase().contains("dns") {
                NetworkError::ConnectionError(error_msg)
            } else {
                NetworkError::Unknown(error_msg)
            }
        }
    }
}

impl From<serde_json::Error> for NetworkError {
    fn from(err: serde_json::Error) -> Self {
        NetworkError::JsonError(err.to_string())
    }
}

impl From<url::ParseError> for NetworkError {
    fn from(err: url::ParseError) -> Self {
        NetworkError::InvalidUrl(err.to_string())
    }
}

