use crate::user_domain_model::UserListDomainModel;
use user_data::UserDataModel;

/// Mapper to convert from Data layer models to Domain layer models
/// This mapper is in the Domain layer because:
/// - Domain layer controls how Data models are converted to Domain models
/// - Domain defines what it needs from Data
/// - Keeps Domain layer independent and in control
pub struct UserDataToDomainMapper;

impl UserDataToDomainMapper {
    /// Map Data layer UserDataModel to Domain layer UserListDomainModel
    pub fn map(data_user: &UserDataModel) -> UserListDomainModel {
        UserListDomainModel::new(
            data_user.id,
            data_user.first_name.clone(),
            data_user.last_name.clone(),
            data_user.email.clone(),
            data_user.phone.clone(),
            data_user.age,
        )
    }

    /// Map a collection of Data layer UserDataModels to Domain layer UserListDomainModels
    pub fn vec_map(data_users: &[UserDataModel]) -> Vec<UserListDomainModel> {
        data_users.iter().map(Self::map).collect()
    }
}

