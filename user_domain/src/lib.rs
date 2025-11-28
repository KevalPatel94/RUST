// UniFFI scaffolding - must be called before any uniffi types
uniffi::setup_scaffolding!();

pub mod get_users_use_case;
pub mod user_data_to_domain_mapper;
pub mod user_domain_model;
pub mod user_domain_record;

// Re-export domain_common types for convenience
pub use domain_common::{BaseUseCase, DomainError, ErrorDisplay};
pub use get_users_use_case::GetUsersUseCaseImpl;
pub use user_data_to_domain_mapper::UserDataToDomainMapper;
pub use user_domain_record::UserDomainModel;
