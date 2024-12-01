mod format;
mod utils;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

#[derive(Serialize, Deserialize, Debug)]
struct Translations {
    translations: HashMap<String, Value>,
}

static TRANSLATIONS: Lazy<Mutex<Translations>> = Lazy::new(|| {
    Mutex::new(Translations {
        translations: HashMap::new(),
    })
});

#[wasm_bindgen]
pub fn set_translations(locale: &str, json: &str) -> Result<(), JsValue> {
    let mut translations = TRANSLATIONS.lock().unwrap();
    let parsed: Value = serde_json::from_str(json).map_err(|e| JsValue::from_str(&e.to_string()))?;

    if let Some(existing) = translations.translations.get_mut(locale) {
        if let (Value::Object(existing_map), Value::Object(new_map)) = (existing, &parsed) {
            for (key, value) in new_map {
                existing_map.insert(key.clone(), value.clone());
            }
        }
    } else {
        translations.translations.insert(locale.to_string(), parsed);
    }

    Ok(())
}

#[wasm_bindgen]
pub fn set_translations_from_object(locale: &str, obj: JsValue) -> Result<(), JsValue> {
    let json: Value = from_value(obj)?;
    let json_str = serde_json::to_string(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
    set_translations(locale, &json_str)
}

#[wasm_bindgen]
pub fn get_translations(locale: &str) -> Result<String, JsValue> {
    let translations = TRANSLATIONS.lock().unwrap();
    let translation = translations.translations.get(locale).ok_or(JsValue::from_str("Locale not found"))?;
    let json_str = serde_json::to_string(translation).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(json_str)
}

#[wasm_bindgen]
pub fn del_translations(locale: &str) -> Result<(), JsValue> {
    let mut translations = TRANSLATIONS.lock().unwrap();
    translations.translations.remove(locale);
    Ok(())
}

#[wasm_bindgen]
pub fn del_translation(locale: &str, key: &str) -> Result<(), JsValue> {
    let mut translations = TRANSLATIONS.lock().unwrap();
    if let Some(existing) = translations.translations.get_mut(locale) {
        if let Value::Object(existing_map) = existing {
            let keys: Vec<&str> = key.split('.').collect();
            let mut current = existing_map;
            for k in keys.iter().take(keys.len() - 1) {
                if let Some(Value::Object(next_map)) = current.get_mut(*k) {
                    current = next_map;
                } else {
                    return Ok(()); // Key path does not exist, nothing to delete
                }
            }
            // Convert the last key to a String before removing it
            if let Some(last_key) = keys.last() {
                current.remove(&last_key.to_string());
            }
        }
    }
    Ok(())
}

#[wasm_bindgen]
pub fn get_translation(locale: &str, key: &str) -> Result<String, JsValue> {
    let translations = TRANSLATIONS.lock().unwrap();
    let translation = translations.translations.get(locale).ok_or("Locale not found")?;
    let value = get_value_by_key(translation, key)?;
    Ok(value.to_string())
}

fn get_value_by_key(value: &Value, key: &str) -> Result<Value, JsValue> {
    let keys: Vec<&str> = key.split('.').collect();
    let mut current = value;
    for k in keys {
        current = current.get(k).ok_or(format!("Key {} not found", k))?;
    }
    Ok(current.as_str().into())
}

#[wasm_bindgen]
pub fn has_translation(locale: &str, key: &str) -> bool {
    let translations = TRANSLATIONS.lock().unwrap();
    if let Some(translation) = translations.translations.get(locale) {
        get_value_by_key(translation, key).is_ok()
    } else {
        false
    }
}

#[wasm_bindgen]
pub fn has_locale(locale: &str) -> bool {
    let translations = TRANSLATIONS.lock().unwrap();
    translations.translations.contains_key(locale)
}

#[wasm_bindgen]
pub fn format_translation(locale: &str, key: &str, args: JsValue) -> Result<String, JsValue> {
    let translations = TRANSLATIONS.lock().unwrap();
    let translation = translations.translations.get(locale).ok_or("Locale not found")?;
    let value = get_value_by_key(translation, key)?;
    let value_str = value.as_str().ok_or("Translation is not a string")?;
    let args_map: HashMap<String, String> = from_value(args)?;
    let formatted = format::format_string(value_str, &args_map)?;
    Ok(formatted)
}

#[wasm_bindgen]
pub async fn load_translations(url: &str) -> Result<(), JsValue> {
    let opts = RequestInit::new();
    opts.set_method("GET");

    let request = Request::new_with_str_and_init(url, &opts)?;
    let window = web_sys::window().ok_or("Window not found")?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    let json = JsFuture::from(resp.json()?).await?;
    let translations: HashMap<String, Value> = from_value(json)?;

    let mut locked_translations = TRANSLATIONS.lock().unwrap();
    for (locale, translation) in translations {
        locked_translations.translations.insert(locale, translation);
    }

    Ok(())
}

#[wasm_bindgen]
pub async fn err(e: &str) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    eprintln!("Failed to parse JSON: {}", e);

    Ok(())
}

#[wasm_bindgen]
pub fn get_all_locales() -> Result<JsValue, JsValue> {
    let translations = TRANSLATIONS.lock().unwrap();
    let locales: Vec<String> = translations.translations.keys().cloned().collect();
    to_value(&locales).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn get_all_translations_for_locale(locale: &str) -> Result<JsValue, JsValue> {
    let translations = TRANSLATIONS.lock().unwrap();
    let translation = translations.translations.get(locale).ok_or("Locale not found")?;
    to_value(translation).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn clear_all_translations() -> Result<(), JsValue> {
    let mut translations = TRANSLATIONS.lock().unwrap();
    translations.translations.clear();
    Ok(())
}

#[wasm_bindgen]
pub fn update_translation(locale: &str, key: &str, value: JsValue) -> Result<(), JsValue> {
    let mut translations = TRANSLATIONS.lock().unwrap();
    if let Some(existing) = translations.translations.get_mut(locale) {
        if let Value::Object(existing_map) = existing {
            let keys: Vec<&str> = key.split('.').collect();
            let mut current = existing_map;
            for k in keys.iter().take(keys.len() - 1) {
                if let Some(Value::Object(next_map)) = current.get_mut(*k) {
                    current = next_map;
                } else {
                    return Err(JsValue::from_str("Key path does not exist"));
                }
            }
            // Convert the last key to a String before inserting it
            if let Some(last_key) = keys.last() {
                let value: Value = from_value(value)?;
                current.insert(last_key.to_string(), value);
            }
        }
    } else {
        return Err(JsValue::from_str("Locale not found"));
    }
    Ok(())
}

#[wasm_bindgen]
pub fn get_all_translations() -> Result<JsValue, JsValue> {
    let translations = TRANSLATIONS.lock().unwrap();
    to_value(&translations.translations).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn has_key_in_translations(locale: &str, key: &str) -> bool {
    let translations = TRANSLATIONS.lock().unwrap();
    if let Some(translation) = translations.translations.get(locale) {
        let keys: Vec<&str> = key.split('.').collect();
        let mut current = translation;
        for k in keys {
            if let Some(next) = current.get(k) {
                current = next;
            } else {
                return false;
            }
        }
        true
    } else {
        false
    }
}