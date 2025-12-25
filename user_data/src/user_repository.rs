use user_data_source::{UserDataModel, UsersResponse};
use async_trait::async_trait;
use std::sync::Arc;
use user_data_source::{UserNetworkDataSource, UserNetworkDataSourceImpl, UserDataSourceError};
use repository_common::RepositoryCommonError;
use thiserror::Error;

/// Error type for UserRepository operations
#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("Data source error: {0}")]
    DataSource(#[from] UserDataSourceError),
    #[error("Repository error: {0}")]
    Common(#[from] RepositoryCommonError),
    #[error("Invalid user data: {0}")]
    InvalidData(String),
}

/// Trait for user data repository operations
/// Repository orchestrates data sources and applies extensions for processing
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Fetch all users from data sources
    async fn get_users(&self) -> Result<UsersResponse, UserRepositoryError>;
    
    /// Fetch a single user by ID from data sources
    async fn get_user_by_id(&self, id: u64) -> Result<UserDataModel, UserRepositoryError>;
}

/// Concrete implementation of UserRepository using data sources
/// Repository orchestrates data sources (network, local, cache, etc.)
pub struct UserRepositoryImpl {
    network_data_source: Arc<dyn UserNetworkDataSource>,
}

impl UserRepositoryImpl {
    /// Create a new UserRepositoryImpl with default network data source
    pub fn new() -> Result<Self, UserRepositoryError> {
        let network_data_source = UserNetworkDataSourceImpl::new()
            .map_err(UserRepositoryError::DataSource)?;
        
        Ok(Self {
            network_data_source: Arc::new(network_data_source),
        })
    }

    /// Create a new UserRepositoryImpl with a custom network data source
    pub fn with_network_source(data_source: Arc<dyn UserNetworkDataSource>) -> Self {
        Self {
            network_data_source: data_source,
        }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn get_users(&self) -> Result<UsersResponse, UserRepositoryError> {
        // Use network data source to fetch users
        // In the future, can add fallback to local/cache sources here
        self.network_data_source
            .get_users()
            .await
            .map_err(UserRepositoryError::DataSource)
    }

    async fn get_user_by_id(&self, id: u64) -> Result<UserDataModel, UserRepositoryError> {
        // Use network data source to fetch single user
        // In the future, can add fallback to local/cache sources here
        self.network_data_source
            .get_user_by_id(id)
            .await
            .map_err(UserRepositoryError::DataSource)
    }
}

