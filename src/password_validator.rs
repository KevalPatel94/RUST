use std::time::Duration;
use async_std::future::{timeout, pending};
use crate::localizer;

#[derive(uniffi::Enum)]
#[derive(PartialEq, Debug)]
pub enum PasswordValidation {
  TooShort,
  TooLong,
  NoUppercase,
  NoLowercase,
  NoNumber,
  NoSymbol,
  Valid
}

#[derive(uniffi::Object)]
pub struct PasswordValidator {}

#[uniffi::export]
impl PasswordValidator {

    #[uniffi::constructor]
    pub fn new() -> Self {
        PasswordValidator {  }
    }

pub fn validate_with_message(&self, password: String) -> String {
    if password.len() < 8 { 
        localizer::get_localized_text("atleast-eight-characters".to_string())
    } else if password.len() > 20 {
        localizer::get_localized_text("no-more-than-twenty-characters".to_string())
    } else if !password.chars().any(|c| c.is_ascii_uppercase()) {
        localizer::get_localized_text("password-no-uppercase".to_string())
    } else if !password.chars().any(|c| c.is_ascii_lowercase()) {
        localizer::get_localized_text("password-no-lowercase".to_string())
    } else if !password.chars().any(|c| c.is_ascii_digit()) {
        localizer::get_localized_text("password-no-number".to_string())
    } else if !password.chars().any(|c| "~!@#$%^&*()^&+=".contains(c)) {
        localizer::get_localized_text("password-no-symbol".to_string())
    } else {
        localizer::get_localized_text("password-valid".to_string())
    }
}

pub fn validate(&self, password: String) -> PasswordValidation {
    //  i18n::init_i18n();
    // Set language to Spanish for this example
    // i18n::set_global_locale("en-US");
    if password.len() < 8 { 
        PasswordValidation::TooShort
    } else if password.len() > 20 {
        PasswordValidation::TooLong
    } else if !password.chars().any(|c| c.is_ascii_uppercase()) {
        PasswordValidation::NoUppercase
    } else if !password.chars().any(|c| c.is_ascii_lowercase()) {
        PasswordValidation::NoLowercase
    } else if !password.chars().any(|c| c.is_ascii_digit()) {
        PasswordValidation::NoNumber
    } else if !password.chars().any(|c| "~!@#$%^&*()^&+=".contains(c)) {
        PasswordValidation::NoSymbol
    } else {
        PasswordValidation::Valid
    }
}

pub async fn say_after(&self,ms: u64, who: String) -> String {
    let never = pending::<()>();
    timeout(Duration::from_millis(ms), never).await.unwrap_err();
    format!("Hello, {who}!")
}

pub fn validate_password_message_non_localized(&self, password: String) -> String {
    // Return plain English messages without going through localization.
    if password.len() < 8 {
        "Password must be at least 8 characters".to_string()
    } else if password.len() > 20 {
        "Password must be no more than 20 characters".to_string()
    } else if !password.chars().any(|c| c.is_ascii_uppercase()) {
        "Password must contain at least 1 uppercase letter".to_string()
    } else if !password.chars().any(|c| c.is_ascii_lowercase()) {
        "Password must contain at least 1 lowercase letter".to_string()
    } else if !password.chars().any(|c| c.is_ascii_digit()) {
        "Password must contain at least 1 number".to_string()
    } else if !password.chars().any(|c| "~!@#$%^&*()^&+=".contains(c)) {
        "Password must contain at least 1 symbol (~!@#$%^&*()^&+=)".to_string()
    } else {
        "Password is valid".to_string()
    }
}

pub fn validate_passwords_score_repeated(&self, inputs: Vec<String>, rounds: u32) -> u64 {
        let mut score: u64 = 0;
        for _ in 0..rounds {
            for p in inputs.iter() {
                let inc = match self.validate(p.clone()) {
                    PasswordValidation::TooShort => 1,
                    PasswordValidation::TooLong => 2,
                    PasswordValidation::NoUppercase => 3,
                    PasswordValidation::NoLowercase => 4,
                    PasswordValidation::NoNumber => 5,
                    PasswordValidation::NoSymbol => 6,
                    PasswordValidation::Valid => 7,
                };
                score += inc;
            }
        }
        score
}

    pub fn validate_passwords_count_valid(&self, inputs: Vec<String>, rounds: u32) -> u64 {
        let mut valid_count: u64 = 0;
        for _ in 0..rounds {
            for p in inputs.iter() {
                if matches!(self.validate(p.clone()), PasswordValidation::Valid) {
                    valid_count += 1;
                }
            }
        }
        valid_count
    }

/// Returns an aggregated policy message if the password violates any rule.
/// If all rules are satisfied, returns an empty string.
pub fn old_password_policy(&self, password: String) -> String {
    let length_ok = (8..=20).contains(&password.len());
    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_number = password.chars().any(|c| c.is_ascii_digit());
    let has_symbol = password.chars().any(|c| "~!@#$%^&*()^&+=".contains(c));
  
    if length_ok && has_upper && has_lower && has_number && has_symbol {
      "Valid Password".to_string()
    } else {
      "Enter 8-20 characters\nUse a number and symbol (~!@#$%^&*()^&+=)\nUse an uppercase and lowercase letter".to_string()
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_too_short_message() {
        let validator = PasswordValidator::new();
        assert_eq!(validator.validate_with_message("Ab1!".to_string()), "Password must be at least 8 characters");
    }

    #[test]
    fn test_too_long_message() {
        let validator = PasswordValidator::new();
        let long = "A1!aaaaaaaaaaaaaaaaaaaa".to_string();
        assert_eq!(validator.validate_with_message(long), "Password must be no more than 20 characters");
    }

    #[test]
    fn test_no_uppercase_message() {
        let validator = PasswordValidator::new();
        assert_eq!(validator.validate_with_message("abc1!abc".to_string()), "Password must contain at least 1 uppercase letter");
    }

    #[test]
    fn test_no_lowercase_message() {
        let validator = PasswordValidator::new();
        assert_eq!(validator.validate_with_message("ABC1!ABC".to_string()), "Password must contain at least 1 lowercase letter");
    }

    #[test]
    fn test_no_number_message() {
        let validator = PasswordValidator::new();
        assert_eq!(validator.validate_with_message("Abc!Abcd".to_string()), "Password must contain at least 1 number");
    }

    #[test]
    fn test_no_symbol_message() {
        let validator = PasswordValidator::new();
        assert_eq!(validator.validate_with_message("Abc1Abcd".to_string()), "Password must contain at least 1 symbol (~!@#$%^&*()^&+=)");
    }

    #[test]
    fn test_valid_message() {
        let validator = PasswordValidator::new();
        assert_eq!(validator.validate_with_message("Abc1!abc".to_string()), "Password is valid");
    }
}
