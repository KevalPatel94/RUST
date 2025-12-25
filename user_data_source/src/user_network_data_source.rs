use async_trait::async_trait;
use network::HTTPClientImpl;
use repository_common::{HttpRepositoryHelper, RepositoryCommonError};
use crate::{UserDataModel, UsersResponse};
use crate::error::UserDataSourceError;

/// Trait for user network data source operations
/// This trait defines the contract for fetching user data from network sources
#[async_trait]
pub trait UserNetworkDataSource: Send + Sync {
    /// Fetch all users from the network
    async fn get_users(&self) -> Result<UsersResponse, UserDataSourceError>;
    
    /// Fetch a single user by ID from the network
    async fn get_user_by_id(&self, id: u64) -> Result<UserDataModel, UserDataSourceError>;
}

/// Concrete implementation of UserNetworkDataSource using HTTP client
pub struct UserNetworkDataSourceImpl {
    helper: HttpRepositoryHelper,
}

impl UserNetworkDataSourceImpl {
    /// Create a new UserNetworkDataSourceImpl with default configuration
    pub fn new() -> Result<Self, UserDataSourceError> {
        let client = HTTPClientImpl::new()
            .map_err(RepositoryCommonError::Network)?;
        
        let helper = HttpRepositoryHelper::new(
            client,
            "https://dummyjson.com".to_string(),
        );
        
        Ok(Self { helper })
    }

    /// Create a new UserNetworkDataSourceImpl with a custom base URL
    pub fn with_base_url(client: HTTPClientImpl, base_url: String) -> Self {
        Self {
            helper: HttpRepositoryHelper::new(client, base_url),
        }
    }

    /// Create a new UserNetworkDataSourceImpl with a custom HTTP client and base URL
    pub fn with_config(client: HTTPClientImpl, base_url: String) -> Self {
        Self {
            helper: HttpRepositoryHelper::new(client, base_url),
        }
    }
}

#[async_trait]
impl UserNetworkDataSource for UserNetworkDataSourceImpl {
    async fn get_users(&self) -> Result<UsersResponse, UserDataSourceError> {
        // Fetch users from network using HTTP helper
        self.helper
            .get("users")
            .await
            .map_err(UserDataSourceError::Common)
    }

    async fn get_user_by_id(&self, id: u64) -> Result<UserDataModel, UserDataSourceError> {
        // Fetch single user from network using HTTP helper
        self.helper
            .get(&format!("users/{}", id))
            .await
            .map_err(UserDataSourceError::Common)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_user_network_data_source_creation() {
        let data_source = UserNetworkDataSourceImpl::new();
        assert!(data_source.is_ok());
    }

    #[tokio::test]
    async fn test_user_network_data_source_with_custom_url() {
        let client = HTTPClientImpl::new().unwrap();
        let _data_source = UserNetworkDataSourceImpl::with_base_url(
            client,
            "https://api.example.com".to_string(),
        );
        // Verify it creates successfully (no panic)
    }
}

