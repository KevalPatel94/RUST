/// Simple error display configuration for platforms
/// Contains only what platforms need to display errors
#[derive(Debug, Clone, uniffi::Record)]
pub struct ErrorDisplay {
    pub title: String,
    pub subtitle: String,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct EpmtyDataModel {
    pub title: String,
    pub subtitle: String,
    pub button_title: String
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

