/// Presentation model for displaying users in the UI
/// This model is optimized for platform UI display (Android/iOS)
#[derive(Debug, Clone, uniffi::Record)]
pub struct UserPresentationModel {
    pub id: u64,
    pub display_name: String,
    pub email: String,
    pub phone: String,
    pub age: u32,
    pub age_display: String, // Formatted for display (e.g., "29 years old")
}

impl UserPresentationModel {
    /// Create a new UserPresentationModel from domain model
    pub fn from_domain(domain_user: &user_domain::UserDomainModel) -> Self {
        Self {
            id: domain_user.id,
            display_name: domain_user.display_name().to_string(),
            email: domain_user.email.clone(),
            phone: domain_user.phone.clone(),
            age: domain_user.age,
            age_display: format!("{} years old", domain_user.age),
        }
    }

    /// Create a list of UserPresentationModel from domain models
    pub fn from_domain_vec(domain_users: &[user_domain::UserDomainModel]) -> Vec<Self> {
        domain_users.iter().map(Self::from_domain).collect()
    }
}

