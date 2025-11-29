use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use fluent::{FluentBundle, FluentResource};
use unic_langid::LanguageIdentifier;

// Pre-loaded translations cache to avoid Fluent bundle thread safety issues
static TRANSLATIONS: OnceLock<HashMap<String, HashMap<String, String>>> = OnceLock::new();
static CURRENT_LOCALE: Mutex<String> = Mutex::new(String::new());

fn get_translations() -> &'static HashMap<String, HashMap<String, String>> {
    TRANSLATIONS.get_or_init(|| {
        let mut translations = HashMap::new();
        
        // Load translations from FTL files
        if let Ok(en_translations) = load_translations_from_ftl("en-US") {
            translations.insert("en".to_string(), en_translations);
        }
        
        if let Ok(es_translations) = load_translations_from_ftl("es") {
            translations.insert("es".to_string(), es_translations);
        }
        
        if let Ok(fr_translations) = load_translations_from_ftl("fr") {
            translations.insert("fr".to_string(), fr_translations);
        }
        
        translations
    })
}



fn load_fluent_bundle_from_content(locale: &str, ftl_content: &str) -> Result<FluentBundle<FluentResource>, Box<dyn std::error::Error>> {
    // Parse the FTL content
    let resource = FluentResource::try_new(ftl_content.to_string())
        .map_err(|e| format!("Failed to parse FTL content: {:?}", e))?;
    
    // Create language identifier
    let lang_id: LanguageIdentifier = locale.parse()
        .map_err(|e| format!("Failed to parse language ID {}: {}", locale, e))?;
    
    // Create and configure bundle
    let mut bundle = FluentBundle::new(vec![lang_id]);
    bundle.add_resource(resource)
        .map_err(|e| format!("Failed to add resource to bundle: {:?}", e))?;
    
    Ok(bundle)
}

fn load_translations_from_ftl(locale: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    // Embed FTL files at compile time for mobile deployment
    let ftl_content = match locale {
        "en-US" => include_str!("locales/en-US/main.ftl"),
        "es" => include_str!("locales/es/main.ftl"),  
        "fr" => include_str!("locales/fr/main.ftl"),
        _ => return Err(format!("Unsupported locale: {}", locale).into()),
    };
    
    let bundle = load_fluent_bundle_from_content(locale, ftl_content)?;
    
    // Extract all messages dynamically by parsing the FTL content
    let mut translations = HashMap::new();
    
    // Parse FTL content to find all message keys
    for line in ftl_content.lines() {
        let line = line.trim();
        // Look for message definitions (key = value)
        if !line.is_empty() && !line.starts_with('#') && line.contains('=') 
            && let Some(key) = line.split('=').next() {
            let key = key.trim();
            // Skip complex messages with special syntax
            if line.contains("->") || line.contains("[") {
                continue; // Skip plural/selector messages
            }
            
            // Get the translation from the bundle
            if let Some(message) = bundle.get_message(key)
                && let Some(pattern) = message.value() {
                let mut errors = vec![];
                let value = bundle.format_pattern(pattern, None, &mut errors);
                
                if key == "greeting" {
                    // Special handling for parameterized messages
                    let simple_template = value.to_string()
                        .replace(['\u{2068}'], "") // First-strong isolate  
                        .replace('\u{2069}', ""); // Pop directional isolate
                    translations.insert(key.to_string(), simple_template);
                } else {
                    // Regular simple messages
                    translations.insert(key.to_string(), value.to_string());
                }
            }
        }
    }
    
    Ok(translations)
}

// Global functions for easy access
#[cfg_attr(not(target_arch = "wasm32"), uniffi::export)]
pub fn init_localization() {
    // Initialize translations (calling this loads them)
    let _ = get_translations();
    
    // Set default locale
    if let Ok(mut locale) = CURRENT_LOCALE.lock() {
        *locale = "en".to_string();
    }
}

#[cfg_attr(not(target_arch = "wasm32"), uniffi::export)]
pub fn set_global_locale(locale: String) {
    if let Ok(mut current_locale) = CURRENT_LOCALE.lock() {
        *current_locale = locale;
    }
}

#[cfg_attr(not(target_arch = "wasm32"), uniffi::export)]
pub fn get_localized_text(key: String) -> String {
    let translations = get_translations();
    
    let current_locale = if let Ok(locale) = CURRENT_LOCALE.lock() {
        locale.clone()
    } else {
        "en".to_string()
    };
    
    if let Some(locale_translations) = translations.get(&current_locale)
        && let Some(translation) = locale_translations.get(&key) {
        return translation.clone();
    }
    
    // Fallback to English
    if let Some(fallback) = translations.get("en")
        && let Some(translation) = fallback.get(&key) {
        return translation.clone();
    }
    format!("[{}]", key)
}

#[cfg_attr(not(target_arch = "wasm32"), uniffi::export)]
pub fn get_localized_text_with_params(key: String, params: HashMap<String, String>) -> String {
    let mut text = get_localized_text(key);
    
    // Handle Fluent-style parameters (both {$param} and { $param } formats)
    for (param_key, param_value) in params {
        // Fluent format with $ (no spaces)
        let fluent_placeholder_compact = format!("{{${}}}", param_key);
        text = text.replace(&fluent_placeholder_compact, &param_value);
        
        // Fluent format with $ (with spaces)
        let fluent_placeholder_spaced = format!("{{ ${} }}", param_key);
        text = text.replace(&fluent_placeholder_spaced, &param_value);
        
        // Simple format without $ (with spaces)
        let simple_placeholder = format!("{{ {} }}", param_key);
        text = text.replace(&simple_placeholder, &param_value);
    }
    
    text
}