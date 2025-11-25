#[cfg(all(target_arch = "wasm32", feature = "js"))]
mod wasm_bindings {
    use std::collections::HashMap;
    use wasm_bindgen::prelude::*;
    use crate::password_validator::PasswordValidator;
    use crate::localizer;
    use js_sys::Array;

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
    }
}


