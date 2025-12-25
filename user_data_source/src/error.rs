use thiserror::Error;
use repository_common::RepositoryCommonError;

/// Error type for UserDataSource operations
#[derive(Debug, Error)]
pub enum UserDataSourceError {
    #[error("Data source error: {0}")]
    Common(#[from] RepositoryCommonError),
    #[error("Invalid user data: {0}")]
    InvalidData(String),
    #[error("Data source unavailable: {0}")]
    Unavailable(String),
}

