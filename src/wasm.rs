#[cfg(all(target_arch = "wasm32", feature = "js"))]
mod wasm_bindings {
    use std::collections::HashMap;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{Request, RequestInit, RequestMode, Response};
    use crate::password_validator::PasswordValidator;
    use crate::localizer;
    use js_sys::{Array, Promise};
    use serde::{Deserialize, Serialize};

    // Arithmetic
    #[wasm_bindgen]
    pub fn add(a: u64, b: u64) -> u64 {
        crate::add(a, b)
    }

    #[wasm_bindgen]
    pub fn difference(a: u64, b: u64) -> u64 {
        crate::difference(a, b)
    }

    // Localization
    #[wasm_bindgen]
    pub fn initialize_localization() {
        localizer::init_localization();
    }

    #[wasm_bindgen]
    pub fn set_app_locale(locale: String) {
        localizer::set_global_locale(locale);
    }

    #[wasm_bindgen]
    pub fn get_translated_text(key: String) -> String {
        localizer::get_localized_text(key)
    }

    #[wasm_bindgen]
    pub fn get_rust_demo_title(name: String) -> String {
        let mut params = HashMap::new();
        params.insert("name".to_string(), name);
        localizer::get_localized_text_with_params("rust-demo-title".to_string(), params)
    }

    // WASM-compatible User Domain Models
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WasmUserDomainModel {
        pub id: u64,
        pub first_name: String,
        pub last_name: String,
        pub phone: String,
        pub full_name: String,
        pub image_url: String,
        pub age_display: String,
        pub email_display: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WasmEmptyDataModel {
        pub title: String,
        pub subtitle: String,
        pub button_title: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WasmErrorDisplay {
        pub title: String,
        pub subtitle: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(tag = "type")]
    pub enum WasmUserDomainResultModel {
        #[serde(rename = "loaded")]
        Loaded { data: Vec<WasmUserDomainModel> },
        #[serde(rename = "empty")]
        Empty { data: WasmEmptyDataModel },
        #[serde(rename = "error")]
        Error { display: WasmErrorDisplay },
    }

    // Internal data models for API response
    #[derive(Debug, Clone, Deserialize)]
    struct UserDataModel {
        id: u64,
        #[serde(rename = "firstName")]
        first_name: String,
        #[serde(rename = "lastName")]
        last_name: String,
        age: u32,
        email: String,
        phone: String,
        image: String,
    }

    #[derive(Debug, Clone, Deserialize)]
    struct UsersResponse {
        users: Vec<UserDataModel>,
    }

    // GetUsersUseCase for WASM - uses fetch API directly
    #[wasm_bindgen]
    pub struct WasmGetUsersUseCase {
        api_url: String,
    }

    #[wasm_bindgen]
    impl WasmGetUsersUseCase {
        #[wasm_bindgen(constructor)]
        pub fn new() -> WasmGetUsersUseCase {
            WasmGetUsersUseCase {
                api_url: "https://dummyjson.com/users".to_string(),
            }
        }

        /// Execute the use case to get all users
        /// Returns a Promise that resolves to WasmUserDomainResultModel (as JSON string)
        #[wasm_bindgen]
        pub fn execute(&self) -> Promise {
            let url = self.api_url.clone();
            
            wasm_bindgen_futures::future_to_promise(async move {
                let mut opts = RequestInit::new();
                opts.method("GET");
                opts.set_mode(RequestMode::Cors);

                let request = Request::new_with_str_and_init(&url, &opts)
                    .map_err(|e| JsValue::from_str(&format!("Failed to create request: {:?}", e)))?;

                let window = web_sys::window()
                    .ok_or_else(|| JsValue::from_str("No window object"))?;
                
                let resp_value = JsFuture::from(window.fetch_with_request(&request))
                    .await
                    .map_err(|e| JsValue::from_str(&format!("Fetch failed: {:?}", e)))?;

                let resp: Response = resp_value.dyn_into()
                    .map_err(|e| JsValue::from_str(&format!("Response is not valid: {:?}", e)))?;

                let json = JsFuture::from(resp.json()
                    .map_err(|e| JsValue::from_str(&format!("Failed to get JSON: {:?}", e)))?)
                    .await
                    .map_err(|e| JsValue::from_str(&format!("Failed to parse JSON: {:?}", e)))?;

                // Parse the API response
                let users_response: UsersResponse = serde_wasm_bindgen::from_value(json)
                    .map_err(|e| JsValue::from_str(&format!("Failed to deserialize: {:?}", e)))?;

                // Map to domain models (with pre-formatted strings)
                let domain_users: Vec<WasmUserDomainModel> = users_response.users
                    .iter()
                    .map(|data_user| {
                        let full_name = format!("{} {}", data_user.first_name, data_user.last_name);
                        let age_display = format!("{} years old", data_user.age);
                        let email_display = format!("Email: {}", data_user.email);
                        
                        WasmUserDomainModel {
                            id: data_user.id,
                            first_name: data_user.first_name.clone(),
                            last_name: data_user.last_name.clone(),
                            phone: data_user.phone.clone(),
                            full_name,
                            image_url: data_user.image.clone(),
                            age_display,
                            email_display,
                        }
                    })
                    .collect();

                // Determine result state (business logic)
                let result = if domain_users.is_empty() {
                    WasmUserDomainResultModel::Empty {
                        data: WasmEmptyDataModel {
                            title: "No Users".to_string(),
                            subtitle: "There are no users available.".to_string(),
                            button_title: "Refresh".to_string(),
                        },
                    }
                } else {
                    WasmUserDomainResultModel::Loaded { data: domain_users }
                };

                // Convert to JsValue for JavaScript
                serde_wasm_bindgen::to_value(&result)
                    .map_err(|e| JsValue::from_str(&format!("Failed to serialize: {:?}", e)))
            })
        }
    }

    // Password validator wrapper
    #[wasm_bindgen]
    pub struct WasmPasswordValidator {
        inner: PasswordValidator,
    }

    #[wasm_bindgen]
    impl WasmPasswordValidator {
        #[wasm_bindgen(constructor)]
        pub fn new() -> WasmPasswordValidator {
            WasmPasswordValidator { inner: PasswordValidator::new() }
        }

        pub fn validate(&self, password: String) -> String {
            self.inner.validate_with_message(password)
        }

        /// Batch scoring benchmark helper mirroring native API:
        /// sums a small integer per password over N rounds.
        #[wasm_bindgen]
        pub fn validate_passwords_score_repeated(&self, inputs: Array, rounds: u32) -> u64 {
            let mut vec_inputs: Vec<String> = Vec::with_capacity(inputs.length() as usize);
            for v in inputs.iter() {
                if let Some(s) = v.as_string() {
                    vec_inputs.push(s);
                }
            }
            self.inner.validate_passwords_score_repeated(vec_inputs, rounds)
        }

        /// Batch validation that counts valid passwords over N rounds.
        #[wasm_bindgen]
        pub fn validate_passwords_count_valid(&self, inputs: Array, rounds: u32) -> u64 {
            let mut vec_inputs: Vec<String> = Vec::with_capacity(inputs.length() as usize);
            for v in inputs.iter() {
                if let Some(s) = v.as_string() {
                    vec_inputs.push(s);
                }
            }
            self.inner.validate_passwords_count_valid(vec_inputs, rounds)
        }
    }
}


