use domain_common::{ErrorDisplay, EpmtyDataModel};
use crate::user_domain_model::UserDomainModel;

/// Result model for all user domain operations
/// Represents three possible states: Loaded, Empty, or Error
/// 
/// **Why this concrete type:**
/// - UniFFI doesn't support generic types, so we need a concrete enum
/// - UniFFI requires #[derive(uniffi::Enum)] for FFI bindings
#[derive(Debug, Clone, uniffi::Enum)]
pub enum UserDomainResultModel {
    /// Operation succeeded with data (always a vector)
    Loaded { data: Vec<UserDomainModel> },
    /// Operation succeeded but returned empty data
    Empty { data: EpmtyDataModel },
    /// Operation failed with error information
    Error { display: ErrorDisplay },
}
