uniffi::setup_scaffolding!();

pub mod localizer;
pub mod password_validator;
use password_validator::PasswordValidator;

// Re-export domain types for UniFFI
pub use user_domain::{GetUsersUseCaseImpl, UserDomainModel, DomainError, ErrorDisplay};

// Only include JS/WASM exports when building for wasm32 with the "js" feature
#[cfg(all(target_arch = "wasm32", feature = "js"))]
pub mod wasm;


#[uniffi::export]
fn add(a: u64, b: u64) -> u64 {
    a + b
}

#[uniffi::export]
fn difference(a: u64, b: u64) -> u64 {
    a.abs_diff(b)
}

// Localization exports for mobile platforms
#[uniffi::export]
pub fn initialize_localization() {
    localizer::init_localization();
}

#[uniffi::export]
pub fn set_app_locale(locale: String) {
    localizer::set_global_locale(locale);
}

#[uniffi::export]
pub fn get_translated_text(key: String) -> String {
    localizer::get_localized_text(key)
}

#[uniffi::export]
pub fn get_rust_demo_title(name: String) -> String {
    let mut params = std::collections::HashMap::new();
    params.insert("name".to_string(), name);
    localizer::get_localized_text_with_params("rust-demo-title".to_string(), params)
}

// Example function that demonstrates localized password validation
#[uniffi::export]
pub fn validate_password_localized(password: String) -> String {
    let validator = PasswordValidator::new();
    validator.validate_with_message(password)
}