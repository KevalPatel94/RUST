use serde::{Deserialize, Serialize};

/// User data model representing a user from the DummyJSON API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDataModel {
    pub id: u64,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "maidenName", default)]
    pub maiden_name: Option<String>,
    pub age: u32,
    pub gender: String,
    pub email: String,
    pub phone: String,
    pub username: String,
    pub password: String,
    #[serde(rename = "birthDate")]
    pub birth_date: String,
    pub image: String,
    #[serde(rename = "bloodGroup")]
    pub blood_group: Option<String>,
    pub height: Option<f64>,
    pub weight: Option<f64>,
    #[serde(rename = "eyeColor")]
    pub eye_color: Option<String>,
    pub hair: Option<Hair>,
    pub ip: Option<String>,
    pub address: Option<Address>,
    #[serde(rename = "macAddress")]
    pub mac_address: Option<String>,
    pub university: Option<String>,
    pub bank: Option<Bank>,
    pub company: Option<Company>,
    pub ein: Option<String>,
    pub ssn: Option<String>,
    #[serde(rename = "userAgent")]
    pub user_agent: Option<String>,
    pub crypto: Option<Crypto>,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hair {
    pub color: String,
    #[serde(rename = "type")]
    pub hair_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub address: String,
    pub city: String,
    pub state: String,
    #[serde(rename = "stateCode")]
    pub state_code: Option<String>,
    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,
    pub coordinates: Option<Coordinates>,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coordinates {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bank {
    #[serde(rename = "cardExpire")]
    pub card_expire: Option<String>,
    #[serde(rename = "cardNumber")]
    pub card_number: Option<String>,
    #[serde(rename = "cardType")]
    pub card_type: Option<String>,
    pub currency: Option<String>,
    pub iban: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub department: Option<String>,
    pub name: Option<String>,
    pub title: Option<String>,
    pub address: Option<Address>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crypto {
    pub coin: Option<String>,
    pub wallet: Option<String>,
    pub network: Option<String>,
}

/// Response model for the DummyJSON users API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsersResponse {
    pub users: Vec<UserDataModel>,
    pub total: u32,
    pub skip: u32,
    pub limit: u32,
}


