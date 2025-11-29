use crate::user_domain_model::UserDomainModel;
use user_data::UserDataModel;

/// Mapper to convert from Data layer models to Domain layer models
/// This mapper is in the Domain layer because:
/// - Domain layer controls how Data models are converted to Domain models
/// - Domain defines what it needs from Data
/// - Keeps Domain layer independent and in control
pub struct UserDataToDomainMapper;

impl UserDataToDomainMapper {
    /// Map Data layer UserDataModel to Domain layer UserDomainModel
    /// All user-facing strings are generated here in Rust
    /// Only includes fields needed for presentation
    pub fn map(data_user: &UserDataModel) -> UserDomainModel {
        let full_name = format!("{} {}", data_user.first_name, data_user.last_name);
        let age_display = format!("{} years old", data_user.age);
        let email_display = format!("Email: {}", data_user.email);
        
        UserDomainModel {
            id: data_user.id,
            first_name: data_user.first_name.clone(),
            last_name: data_user.last_name.clone(),
            phone: data_user.phone.clone(),
            full_name,
            image_url: data_user.image.clone(),
            age_display,
            email_display,
        }
    }

    /// Map a collection of Data layer UserDataModels to Domain layer UserDomainModels
    pub fn vec_map(data_users: &[UserDataModel]) -> Vec<UserDomainModel> {
        data_users.iter().map(Self::map).collect()
    }
}

