use user_data::{UserRepository, UserRepositoryImpl};
use crate::user_domain_model::UserDomainModel;
use domain_common::{BaseUseCase, DomainError, ErrorDisplay, EpmtyDataModel};
use crate::use_case_result::UserDomainResultModel;
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
    pub fn new() -> Arc<Self> {
        let base = BaseUseCase::new().expect("Failed to create BaseUseCase");
        
        // Use base's runtime handle to initialize repository
        let user_repository = base.runtime_handle().block_on(async {
            UserRepositoryImpl::new().expect("Failed to create UserRepositoryImpl")
        });
        
        Arc::new(Self {
            base,
            user_repository: Arc::new(user_repository),
        })
    }

    /// Execute the use case to get all users
    /// Returns UserDomainResultModel with Loaded, Empty, or Error state
    /// The use case logic determines which case to use based on business rules
    pub async fn execute(&self) -> UserDomainResultModel {
        let repository = Arc::clone(&self.user_repository);
        
        // Use BaseUseCase's execute_async helper - much simpler!
        let result: Result<Vec<UserDomainModel>, DomainError> = self.base.execute_async(async move {
            let users_response = repository
                .get_users()
                .await
                .map_err(|e| {
                    // Preserve error information for debugging
                    // In production, you might want to log this
                    eprintln!("Repository error: {:?}", e);
                    ErrorDisplay {
                        title: "Network Error".to_string(),
                        subtitle: "Please check your internet connection and try again.".to_string(),
                    }
                })?;
            Ok::<Vec<UserDomainModel>, ErrorDisplay>(UserDataToDomainMapper::vec_map(&users_response.users))
        }).await;

        // Use case logic: Determine the appropriate UserDomainResultModel case
        // This is where business rules live - deciding Loaded vs Empty
        match result {
            Ok(users) => {
                // Business logic: Empty list means Empty case, non-empty means Loaded case
                if users.is_empty() {
                    UserDomainResultModel::Empty {
                        data: EpmtyDataModel {
                            title: "No Users".to_string(),
                            subtitle: "There are no users available.".to_string(),
                            button_title: "Refresh".to_string(),
                        },
                    }
                } else {
                    UserDomainResultModel::Loaded { data: users }
                }
            }
            Err(error) => {
                // Error case - convert DomainError to ErrorDisplay
                UserDomainResultModel::Error {
                    display: error.to_display(),
                }
            }
        }
    }
}
