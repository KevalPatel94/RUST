// UniFFI scaffolding - required for UniFFI types (ErrorDisplay, DomainError)
uniffi::setup_scaffolding!();

pub mod base_use_case;
pub mod domain_error;

pub use base_use_case::BaseUseCase;
pub use domain_error::{DomainError, ErrorDisplay};
