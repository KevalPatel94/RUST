use crate::user_list_domain_model::UserListDomainModel;

/// UserDomainModel - exposed via UniFFI for platform use
#[derive(Debug, Clone, uniffi::Record)]
pub struct UserDomainModel {
    pub id: u64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub age: u32,
    pub full_name: String,
    pub image_url: String,
}

impl From<UserListDomainModel> for UserDomainModel {
    fn from(domain: UserListDomainModel) -> Self {
        Self {
            id: domain.id,
            first_name: domain.first_name,
            last_name: domain.last_name,
            email: domain.email,
            phone: domain.phone,
            age: domain.age,
            full_name: domain.full_name,
            image_url: domain.image_url,
        }
    }
}

