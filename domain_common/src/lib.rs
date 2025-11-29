// UniFFI scaffolding - required for UniFFI types (ErrorDisplay, DomainError)
// Only for native platforms, not WASM
#[cfg(not(target_arch = "wasm32"))]
uniffi::setup_scaffolding!();

pub mod base_use_case;
pub mod domain_error;

#[cfg(not(target_arch = "wasm32"))]
pub use base_use_case::BaseUseCase;
pub use domain_error::{DomainError, ErrorDisplay, EpmtyDataModel};
