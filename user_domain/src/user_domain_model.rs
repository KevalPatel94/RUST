/// Domain models - Pure business entities with no external dependencies
/// These models represent the core business logic and should NOT depend on:
/// - Serialization libraries (serde)
/// - Platform-specific types
/// - Data layer types

/// Internal domain User entity (used internally, converted to UserDomainModel for UniFFI)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserListDomainModel {
    pub id: u64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub age: u32,
    pub full_name: String, // Computed/derived field
}

impl UserListDomainModel {
    /// Create a new domain User
    pub fn new(
        id: u64,
        first_name: String,
        last_name: String,
        email: String,
        phone: String,
        age: u32,
    ) -> Self {
        let full_name = format!("{} {}", first_name, last_name);
        Self {
            id,
            first_name,
            last_name,
            email,
            phone,
            age,
            full_name,
        }
    }

    /// Get the user's display name
    pub fn display_name(&self) -> &str {
        &self.full_name
    }
}

