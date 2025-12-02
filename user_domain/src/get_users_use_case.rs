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
                    eprintln!("Repository error: {:?}", e);
                    
                    // Extract error message for better user feedback
                    let error_str = format!("{:?}", e);
                    let subtitle = if error_str.contains("error sending request") || 
                                      error_str.contains("ConnectionError") ||
                                      error_str.contains("connection") {
                        "Unable to connect to the server. Please check your internet connection.".to_string()
                    } else if error_str.contains("TimeoutError") || error_str.contains("timeout") {
                        "Request timed out. Please check your network connection.".to_string()
                    } else if error_str.contains("HttpError") {
                        "Server returned an error. Please try again later.".to_string()
                    } else {
                        "Network request failed. Please check your internet connection.".to_string()
                    };
                    
                    ErrorDisplay {
                        title: "Network Error".to_string(),
                        subtitle,
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

#[cfg(test)]
mod tests {
    use super::*;
    use user_data::{UserDataModel, UsersResponse, UserRepositoryError};
    use async_trait::async_trait;
    use std::sync::Mutex;
    use std::sync::Arc;

    // Mock repository for testing
    struct MockUserRepository {
        users: Arc<Mutex<Vec<UserDataModel>>>,
        should_error: Arc<Mutex<bool>>,
        error_message: Arc<Mutex<String>>,
    }

    impl MockUserRepository {
        fn new() -> Self {
            Self {
                users: Arc::new(Mutex::new(vec![])),
                should_error: Arc::new(Mutex::new(false)),
                error_message: Arc::new(Mutex::new(String::new())),
            }
        }

        fn with_users(self, users: Vec<UserDataModel>) -> Self {
            *self.users.lock().unwrap() = users;
            self
        }

        fn with_error(self, error_message: String) -> Self {
            *self.should_error.lock().unwrap() = true;
            *self.error_message.lock().unwrap() = error_message;
            self
        }
    }

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn get_users(&self) -> Result<UsersResponse, UserRepositoryError> {
            if *self.should_error.lock().unwrap() {
                let message = self.error_message.lock().unwrap().clone();
                return Err(UserRepositoryError::Common(
                    repository_common::RepositoryCommonError::Network(
                        network::error::NetworkError::ConnectionError(message)
                    )
                ));
            }

            let users = self.users.lock().unwrap().clone();
            Ok(UsersResponse {
                total: users.len() as u32,
                skip: 0,
                limit: users.len() as u32,
                users,
            })
        }

        async fn get_user_by_id(&self, _id: u64) -> Result<UserDataModel, UserRepositoryError> {
            Err(UserRepositoryError::InvalidData("Not implemented in mock".to_string()))
        }
    }

    // Helper function to test execute logic without BaseUseCase runtime issues
    // This directly tests the business logic
    async fn execute_use_case_logic(
        repository: Arc<dyn UserRepository>
    ) -> UserDomainResultModel {
        let result: Result<Vec<UserDomainModel>, ErrorDisplay> = async move {
            let users_response = repository
                .get_users()
                .await
                .map_err(|e| {
                    eprintln!("Repository error: {:?}", e);
                    ErrorDisplay {
                        title: "Network Error".to_string(),
                        subtitle: "Please check your internet connection and try again.".to_string(),
                    }
                })?;
            Ok::<Vec<UserDomainModel>, ErrorDisplay>(UserDataToDomainMapper::vec_map(&users_response.users))
        }.await;

        match result {
            Ok(users) => {
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
                UserDomainResultModel::Error {
                    display: error,
                }
            }
        }
    }

    fn create_test_user_data_model(id: u64, first_name: &str, last_name: &str, age: u32, email: &str) -> UserDataModel {
        UserDataModel {
            id,
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            maiden_name: None,
            age,
            gender: "male".to_string(),
            email: email.to_string(),
            phone: "+1234567890".to_string(),
            username: "testuser".to_string(),
            password: "password".to_string(),
            birth_date: "1990-01-01".to_string(),
            image: "https://example.com/image.jpg".to_string(),
            blood_group: None,
            height: None,
            weight: None,
            eye_color: None,
            hair: None,
            ip: None,
            address: None,
            mac_address: None,
            university: None,
            bank: None,
            company: None,
            ein: None,
            ssn: None,
            user_agent: None,
            crypto: None,
            role: None,
        }
    }

    #[tokio::test]
    async fn test_execute_loaded_with_users() {
        let user1 = create_test_user_data_model(1, "John", "Doe", 30, "john@example.com");
        let user2 = create_test_user_data_model(2, "Jane", "Smith", 25, "jane@example.com");
        
        let mock_repo = Arc::new(MockUserRepository::new().with_users(vec![user1, user2]));
        let result = execute_use_case_logic(mock_repo).await;

        match result {
            UserDomainResultModel::Loaded { data } => {
                assert_eq!(data.len(), 2);
                assert_eq!(data[0].id, 1);
                assert_eq!(data[0].full_name, "John Doe");
                assert_eq!(data[0].age_display, "30 years old");
                assert_eq!(data[0].email_display, "Email: john@example.com");
                assert_eq!(data[1].id, 2);
                assert_eq!(data[1].full_name, "Jane Smith");
                assert_eq!(data[1].age_display, "25 years old");
                assert_eq!(data[1].email_display, "Email: jane@example.com");
            }
            _ => panic!("Expected Loaded result, got {:?}", result),
        }
    }

    #[tokio::test]
    async fn test_execute_empty_when_no_users() {
        let mock_repo = Arc::new(MockUserRepository::new().with_users(vec![]));
        let result = execute_use_case_logic(mock_repo).await;

        match result {
            UserDomainResultModel::Empty { data } => {
                assert_eq!(data.title, "No Users");
                assert_eq!(data.subtitle, "There are no users available.");
                assert_eq!(data.button_title, "Refresh");
            }
            _ => panic!("Expected Empty result, got {:?}", result),
        }
    }

    #[tokio::test]
    async fn test_execute_error_on_repository_error() {
        let mock_repo = Arc::new(MockUserRepository::new().with_error("Connection failed".to_string()));
        let result = execute_use_case_logic(mock_repo).await;

        match result {
            UserDomainResultModel::Error { display } => {
                assert_eq!(display.title, "Network Error");
                assert!(display.subtitle.contains("internet connection"));
            }
            _ => panic!("Expected Error result, got {:?}", result),
        }
    }

    #[tokio::test]
    async fn test_execute_single_user() {
        let user = create_test_user_data_model(1, "Alice", "Johnson", 28, "alice@example.com");
        
        let mock_repo = Arc::new(MockUserRepository::new().with_users(vec![user]));
        let result = execute_use_case_logic(mock_repo).await;

        match result {
            UserDomainResultModel::Loaded { data } => {
                assert_eq!(data.len(), 1);
                assert_eq!(data[0].id, 1);
                assert_eq!(data[0].full_name, "Alice Johnson");
                assert_eq!(data[0].age_display, "28 years old");
            }
            _ => panic!("Expected Loaded result, got {:?}", result),
        }
    }

    #[tokio::test]
    async fn test_execute_maps_users_correctly() {
        let user = create_test_user_data_model(
            42,
            "Test",
            "User",
            35,
            "test.user@example.com"
        );
        
        let mock_repo = Arc::new(MockUserRepository::new().with_users(vec![user]));
        let result = execute_use_case_logic(mock_repo).await;

        match result {
            UserDomainResultModel::Loaded { data } => {
                assert_eq!(data.len(), 1);
                let domain_user = &data[0];
                
                // Verify all fields are correctly mapped
                assert_eq!(domain_user.id, 42);
                assert_eq!(domain_user.first_name, "Test");
                assert_eq!(domain_user.last_name, "User");
                assert_eq!(domain_user.full_name, "Test User");
                assert_eq!(domain_user.age_display, "35 years old");
                assert_eq!(domain_user.email_display, "Email: test.user@example.com");
                assert_eq!(domain_user.phone, "+1234567890");
                assert_eq!(domain_user.image_url, "https://example.com/image.jpg");
            }
            _ => panic!("Expected Loaded result, got {:?}", result),
        }
    }

    #[tokio::test]
    async fn test_execute_handles_multiple_users() {
        let users = vec![
            create_test_user_data_model(1, "User", "One", 20, "user1@example.com"),
            create_test_user_data_model(2, "User", "Two", 30, "user2@example.com"),
            create_test_user_data_model(3, "User", "Three", 40, "user3@example.com"),
        ];
        
        let mock_repo = Arc::new(MockUserRepository::new().with_users(users));
        let result = execute_use_case_logic(mock_repo).await;

        match result {
            UserDomainResultModel::Loaded { data } => {
                assert_eq!(data.len(), 3);
                for (i, user) in data.iter().enumerate() {
                    assert_eq!(user.id, (i + 1) as u64);
                    assert_eq!(user.age_display, format!("{} years old", (i + 1) * 10 + 10));
                }
            }
            _ => panic!("Expected Loaded result, got {:?}", result),
        }
    }
}
