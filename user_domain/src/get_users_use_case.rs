use user_data::{UserRepository, UserRepositoryImpl};
use crate::user_domain_model::UserListDomainModel;
use crate::user_domain_record::UserDomainModel;
use domain_common::{BaseUseCase, DomainError, ErrorDisplay};
use crate::user_data_to_domain_mapper::UserDataToDomainMapper;
use std::sync::Arc;

/// GetUsersUseCaseImpl - directly exposed via UniFFI for platform use
/// This is the concrete implementation that platforms will use directly
/// Uses BaseUseCase for common runtime and error handling concerns
#[derive(uniffi::Object)]
pub struct GetUsersUseCaseImpl {
    base: BaseUseCase,
    user_repository: Arc<dyn UserRepository>,
}

#[uniffi::export]
impl GetUsersUseCaseImpl {
    /// Constructor - creates a new GetUsersUseCaseImpl using BaseUseCase
    #[uniffi::constructor]
    pub fn new() -> Result<Arc<Self>, DomainError> {
        let base = BaseUseCase::new()?;
        
        // Use base's runtime handle to initialize repository
        let user_repository = base.runtime_handle().block_on(async {
            UserRepositoryImpl::new()
        })
        .map_err(|_| DomainError::Error {
            display: ErrorDisplay::repository(),
        })?;
        
        Ok(Arc::new(Self {
            base,
            user_repository: Arc::new(user_repository),
        }))
    }

    /// Execute the use case to get all users
    /// Returns UserDomainModel for FFI compatibility
    pub async fn execute(&self) -> Result<Vec<UserDomainModel>, DomainError> {
        let repository = Arc::clone(&self.user_repository);
        
        // Use BaseUseCase's execute_async helper - much simpler!
        self.base.execute_async(async move {
            let users_response = repository
                .get_users()
                .await
                .map_err(|_| ErrorDisplay::repository())?;
            Ok::<Vec<UserListDomainModel>, ErrorDisplay>(UserDataToDomainMapper::vec_map(&users_response.users))
        }).await
        .map(|users| users.into_iter().map(UserDomainModel::from).collect())
    }

    /// Execute the use case to get a user by ID
    /// Returns UserDomainModel for FFI compatibility
    pub async fn execute_by_id(&self, id: u64) -> Result<UserDomainModel, DomainError> {
        // Business logic validation
        if id == 0 {
            return Err(DomainError::Error {
                display: ErrorDisplay::business_logic(
                    "User ID must be greater than 0".to_string(),
                ),
            });
        }

        let repository = Arc::clone(&self.user_repository);
        
        // Use BaseUseCase's execute_async helper - much simpler!
        self.base.execute_async(async move {
            let user_data = repository
                .get_user_by_id(id)
                .await
                .map_err(|_| ErrorDisplay::repository())?;
            Ok::<UserListDomainModel, ErrorDisplay>(UserDataToDomainMapper::map(&user_data))
        }).await
        .map(UserDomainModel::from)
    }

    /// Convert DomainError to ErrorDisplay - platforms use this to get title/subtitle
    pub fn to_error_display(&self, error: DomainError) -> ErrorDisplay {
        error.to_display()
    }
}
