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

#[cfg(test)]
mod tests {
    use super::*;
    use user_data::UserDataModel;

    fn create_test_user_data_model() -> UserDataModel {
        UserDataModel {
            id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            maiden_name: Some("Smith".to_string()),
            age: 30,
            gender: "male".to_string(),
            email: "john.doe@example.com".to_string(),
            phone: "+1234567890".to_string(),
            username: "johndoe".to_string(),
            password: "password123".to_string(),
            birth_date: "1993-01-01".to_string(),
            image: "https://example.com/image.jpg".to_string(),
            blood_group: Some("A+".to_string()),
            height: Some(180.0),
            weight: Some(75.0),
            eye_color: Some("brown".to_string()),
            hair: None,
            ip: Some("192.168.1.1".to_string()),
            address: None,
            mac_address: Some("00:11:22:33:44:55".to_string()),
            university: Some("University".to_string()),
            bank: None,
            company: None,
            ein: Some("12-3456789".to_string()),
            ssn: Some("123-45-6789".to_string()),
            user_agent: Some("Mozilla/5.0".to_string()),
            crypto: None,
            role: Some("user".to_string()),
        }
    }

    #[test]
    fn test_map_single_user() {
        let data_user = create_test_user_data_model();
        let domain_user = UserDataToDomainMapper::map(&data_user);

        assert_eq!(domain_user.id, 1);
        assert_eq!(domain_user.first_name, "John");
        assert_eq!(domain_user.last_name, "Doe");
        assert_eq!(domain_user.phone, "+1234567890");
        assert_eq!(domain_user.full_name, "John Doe");
        assert_eq!(domain_user.image_url, "https://example.com/image.jpg");
        assert_eq!(domain_user.age_display, "30 years old");
        assert_eq!(domain_user.email_display, "Email: john.doe@example.com");
    }

    #[test]
    fn test_map_full_name_formatting() {
        let mut data_user = create_test_user_data_model();
        data_user.first_name = "Jane".to_string();
        data_user.last_name = "Smith".to_string();

        let domain_user = UserDataToDomainMapper::map(&data_user);
        assert_eq!(domain_user.full_name, "Jane Smith");
    }

    #[test]
    fn test_map_age_display_formatting() {
        let test_cases = vec![
            (0, "0 years old"),
            (1, "1 years old"),
            (25, "25 years old"),
            (100, "100 years old"),
        ];

        for (age, expected) in test_cases {
            let mut data_user = create_test_user_data_model();
            data_user.age = age;
            let domain_user = UserDataToDomainMapper::map(&data_user);
            assert_eq!(domain_user.age_display, expected, "Failed for age: {}", age);
        }
    }

    #[test]
    fn test_map_email_display_formatting() {
        let test_cases = vec![
            ("user@example.com", "Email: user@example.com"),
            ("test.email+tag@domain.co.uk", "Email: test.email+tag@domain.co.uk"),
            ("admin@localhost", "Email: admin@localhost"),
        ];

        for (email, expected) in test_cases {
            let mut data_user = create_test_user_data_model();
            data_user.email = email.to_string();
            let domain_user = UserDataToDomainMapper::map(&data_user);
            assert_eq!(domain_user.email_display, expected, "Failed for email: {}", email);
        }
    }

    #[test]
    fn test_map_empty_strings() {
        let mut data_user = create_test_user_data_model();
        data_user.first_name = "".to_string();
        data_user.last_name = "".to_string();
        data_user.email = "".to_string();
        data_user.phone = "".to_string();
        data_user.image = "".to_string();

        let domain_user = UserDataToDomainMapper::map(&data_user);
        assert_eq!(domain_user.full_name, " ");
        assert_eq!(domain_user.email_display, "Email: ");
        assert_eq!(domain_user.phone, "");
        assert_eq!(domain_user.image_url, "");
    }

    #[test]
    fn test_map_special_characters() {
        let mut data_user = create_test_user_data_model();
        data_user.first_name = "José".to_string();
        data_user.last_name = "O'Connor".to_string();
        data_user.email = "josé.o'connor@example.com".to_string();

        let domain_user = UserDataToDomainMapper::map(&data_user);
        assert_eq!(domain_user.full_name, "José O'Connor");
        assert_eq!(domain_user.email_display, "Email: josé.o'connor@example.com");
    }

    #[test]
    fn test_vec_map_empty_list() {
        let data_users: Vec<UserDataModel> = vec![];
        let domain_users = UserDataToDomainMapper::vec_map(&data_users);
        assert_eq!(domain_users.len(), 0);
    }

    #[test]
    fn test_vec_map_single_user() {
        let data_user = create_test_user_data_model();
        let data_users = vec![data_user];
        let domain_users = UserDataToDomainMapper::vec_map(&data_users);

        assert_eq!(domain_users.len(), 1);
        assert_eq!(domain_users[0].id, 1);
        assert_eq!(domain_users[0].full_name, "John Doe");
    }

    #[test]
    fn test_vec_map_multiple_users() {
        let mut user1 = create_test_user_data_model();
        user1.id = 1;
        user1.first_name = "Alice".to_string();
        user1.last_name = "Johnson".to_string();

        let mut user2 = create_test_user_data_model();
        user2.id = 2;
        user2.first_name = "Bob".to_string();
        user2.last_name = "Williams".to_string();

        let mut user3 = create_test_user_data_model();
        user3.id = 3;
        user3.first_name = "Charlie".to_string();
        user3.last_name = "Brown".to_string();

        let data_users = vec![user1, user2, user3];
        let domain_users = UserDataToDomainMapper::vec_map(&data_users);

        assert_eq!(domain_users.len(), 3);
        assert_eq!(domain_users[0].id, 1);
        assert_eq!(domain_users[0].full_name, "Alice Johnson");
        assert_eq!(domain_users[1].id, 2);
        assert_eq!(domain_users[1].full_name, "Bob Williams");
        assert_eq!(domain_users[2].id, 3);
        assert_eq!(domain_users[2].full_name, "Charlie Brown");
    }

    #[test]
    fn test_map_preserves_all_required_fields() {
        let data_user = create_test_user_data_model();
        let domain_user = UserDataToDomainMapper::map(&data_user);

        // Verify all fields are present and correctly mapped
        assert_eq!(domain_user.id, data_user.id);
        assert_eq!(domain_user.first_name, data_user.first_name);
        assert_eq!(domain_user.last_name, data_user.last_name);
        assert_eq!(domain_user.phone, data_user.phone);
        assert_eq!(domain_user.image_url, data_user.image);
        // Verify formatted fields
        assert!(domain_user.full_name.contains(&data_user.first_name));
        assert!(domain_user.full_name.contains(&data_user.last_name));
        assert!(domain_user.age_display.contains(&data_user.age.to_string()));
        assert!(domain_user.email_display.contains(&data_user.email));
    }
}
