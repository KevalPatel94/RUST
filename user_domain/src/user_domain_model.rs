/// UserDomainModel - exposed via UniFFI for platform use
/// All user-facing strings are generated in Rust and included in this model
/// Only contains fields needed for presentation
#[derive(Debug, Clone, uniffi::Record)]
pub struct UserDomainModel {
    pub id: u64,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub full_name: String,
    pub image_url: String,
    /// Age display string formatted as "X years old" - generated in Rust
    pub age_display: String,
    /// Email display string formatted as "Email: user@example.com" - generated in Rust
    pub email_display: String,
}

