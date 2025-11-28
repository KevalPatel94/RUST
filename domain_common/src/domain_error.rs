/// Simple error display configuration for platforms
/// Contains only what platforms need to display errors
#[derive(Debug, Clone, uniffi::Record)]
pub struct ErrorDisplay {
    pub title: String,
    pub subtitle: String,
}

impl ErrorDisplay {
    /// Create a repository/network error
    pub fn repository() -> Self {
        Self {
            title: "Network Error".to_string(),
            subtitle: "Please check your internet connection and try again.".to_string(),
        }
    }

    /// Create a business logic error with custom message
    pub fn business_logic(message: String) -> Self {
        Self {
            title: "Error".to_string(),
            subtitle: message,
        }
    }

    /// Create a generic error
    pub fn generic() -> Self {
        Self {
            title: "Something Went Wrong".to_string(),
            subtitle: "An unexpected error occurred. Please try again.".to_string(),
        }
    }
}

/// Simplified error type for domain operations exposed via UniFFI
/// Uses enum for UniFFI compatibility (Records can't be used as error types in Result)
/// This is just a carrier - platforms convert it to ErrorDisplay to get title/subtitle
#[derive(Debug, Clone, thiserror::Error, uniffi::Error)]
pub enum DomainError {
    #[error("Domain error")]
    Error { display: ErrorDisplay },
}

impl From<ErrorDisplay> for DomainError {
    fn from(display: ErrorDisplay) -> Self {
        DomainError::Error { display }
    }
}

impl DomainError {
    /// Convert to ErrorDisplay - platforms use this to get title/subtitle
    pub fn to_display(&self) -> ErrorDisplay {
        match self {
            DomainError::Error { display } => display.clone(),
        }
    }
}


