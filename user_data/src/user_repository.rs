use crate::user_data_model::{UserDataModel, UsersResponse};
use async_trait::async_trait;
use network::HTTPClientImpl;
use repository_common::{HttpRepositoryHelper, RepositoryCommonError};
use thiserror::Error;

/// Error type for UserRepository operations
#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("Repository error: {0}")]
    Common(#[from] RepositoryCommonError),
    #[error("Invalid user data: {0}")]
    InvalidData(String),
}

/// Trait for user data repository operations
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Fetch all users from the API
    async fn get_users(&self) -> Result<UsersResponse, UserRepositoryError>;
    
    /// Fetch a single user by ID
    async fn get_user_by_id(&self, id: u64) -> Result<UserDataModel, UserRepositoryError>;
}

/// Concrete implementation of UserRepository using shared repository helpers
pub struct UserRepositoryImpl {
    helper: HttpRepositoryHelper,
}

impl UserRepositoryImpl {
    /// Create a new UserRepositoryImpl using shared repository helpers
    pub fn new() -> Result<Self, UserRepositoryError> {
        let client = HTTPClientImpl::new()
            .map_err(RepositoryCommonError::Network)?;
        let helper = HttpRepositoryHelper::new(
            client,
            "https://dummyjson.com".to_string(),
        );
        Ok(Self { helper })
    }

    /// Create a new UserRepositoryImpl with a custom base URL
    pub fn with_base_url(client: HTTPClientImpl, base_url: String) -> Self {
        Self {
            helper: HttpRepositoryHelper::new(client, base_url),
        }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn get_users(&self) -> Result<UsersResponse, UserRepositoryError> {
        // Use shared helper - much simpler!
        self.helper
            .get("users")
            .await
            .map_err(UserRepositoryError::Common)
    }

    async fn get_user_by_id(&self, id: u64) -> Result<UserDataModel, UserRepositoryError> {
        // Use shared helper
        self.helper
            .get(&format!("users/{}", id))
            .await
            .map_err(UserRepositoryError::Common)
    }
}

