use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};
use crate::format::format_string;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub(crate) enum TranslationValue {
    String(String),
    Nested(HashMap<String, TranslationValue>),
}

#[derive(Serialize, Deserialize, Debug)]
struct Translations {
    translations: HashMap<String, HashMap<String, TranslationValue>>,
}

pub struct TranslationManager {
    translations: Mutex<Translations>,
}

impl TranslationManager {
    pub fn new() -> TranslationManager {
        TranslationManager {
            translations: Mutex::new(Translations {
                translations: HashMap::new(),
            }),
        }
    }

    pub fn set_translations(&self, locale: &str, json: &str) -> Result<(), JsValue> {
        let mut translations = self.translations.lock().unwrap();
        let parsed: HashMap<String, TranslationValue> =
            serde_json::from_str(json).map_err(|e| JsValue::from_str(&e.to_string()))?;

        translations
            .translations
            .entry(locale.to_string())
            .or_default()
            .extend(parsed);
        Ok(())
    }

    pub fn get_translation(&self, locale: &str, key: &str) -> Result<JsValue, JsValue> {
        let translations = self.translations.lock().unwrap();
        let translation_map = translations
            .translations
            .get(locale)
            .ok_or_else(|| JsValue::from_str("Locale not found"))?;
        let value = self.get_value_by_key(translation_map, key)?;

        match value {
            TranslationValue::String(s) => Ok(JsValue::from_str(s)),
            TranslationValue::Nested(nested) => to_value(nested).map_err(|e| JsValue::from_str(&e.to_string())),
        }
    }

    pub fn get_translations(&self, locale: &str) -> Result<JsValue, JsValue> {
        let translations = self.translations.lock().unwrap();
        let translation = translations.translations.get(locale).ok_or("Locale not found")?;
        to_value(translation).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn del_translations(&self, locale: &str) -> Result<(), JsValue> {
        let mut translations = self.translations.lock().unwrap();
        translations.translations.remove(locale);
        Ok(())
    }

    pub fn has_translation(&self, locale: &str, key: &str) -> bool {
        let translations = self.translations.lock().unwrap();
        if let Some(translation_map) = translations.translations.get(locale) {
            self.get_value_by_key(translation_map, key).is_ok()
        } else {
            false
        }
    }

    pub fn has_locale(&self, locale: &str) -> bool {
        let translations = self.translations.lock().unwrap();
        translations.translations.contains_key(locale)
    }

    pub fn get_all_locales(&self) -> Result<JsValue, JsValue> {
        let translations = self.translations.lock().unwrap();
        let locales: Vec<String> = translations.translations.keys().cloned().collect();
        to_value(&locales).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn update_translation(&self, locale: &str, key: &str, value: JsValue) -> Result<(), JsValue> {
        let mut translations = self.translations.lock().unwrap();
        if let Some(existing) = translations.translations.get_mut(locale) {
            let parsed_value: TranslationValue = from_value(value)?;
            let keys: Vec<&str> = key.split('.').collect();
            let mut current = existing;
            for k in keys.iter().take(keys.len() - 1) {
                current = match current.get_mut(*k) {
                    Some(TranslationValue::Nested(next_map)) => next_map,
                    _ => return Err(JsValue::from_str("Invalid key path")),
                };
            }
            if let Some(last_key) = keys.last() {
                current.insert(last_key.to_string(), parsed_value);
            }
        } else {
            return Err(JsValue::from_str("Locale not found"));
        }
        Ok(())
    }

    pub fn del_translation(&self, locale: &str, key: &str) -> Result<(), JsValue> {
        let mut translations = self.translations.lock().unwrap();
        if let Some(existing_map) = translations.translations.get_mut(locale) {
            let keys: Vec<&str> = key.split('.').collect();
            let mut current = existing_map;
            for k in keys.iter().take(keys.len() - 1) {
                if let Some(TranslationValue::Nested(next_map)) = current.get_mut(*k) {
                    current = next_map;
                } else {
                    return Ok(()); // Если путь не найден, ничего не удаляем
                }
            }
            if let Some(last_key) = keys.last() {
                current.remove(*last_key);
            }
        }
        Ok(())
    }

    pub fn clear_all_translations(&self) -> Result<(), JsValue> {
        let mut translations = self.translations.lock().unwrap();
        translations.translations.clear();
        Ok(())
    }

    pub async fn load_translations(&self, url: &str) -> Result<(), JsValue> {
        let opts = RequestInit::new();
        opts.set_method("GET");

        let request = Request::new_with_str_and_init(url, &opts)?;
        let window = web_sys::window().ok_or("Window not found")?;
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into()?;
        let json = JsFuture::from(resp.json()?).await?;
        let translations: HashMap<String, HashMap<String, TranslationValue>> = from_value(json)?;

        let mut locked_translations = self.translations.lock().unwrap();
        for (locale, translation) in translations {
            locked_translations.translations.insert(locale, translation);
        }

        Ok(())
    }

    pub fn format_translation(&self, locale: &str, key: &str, args: JsValue) -> Result<String, JsValue> {
        let translations = self.translations.lock().unwrap();
        let translation_map = translations
            .translations
            .get(locale)
            .ok_or_else(|| JsValue::from_str("Locale not found"))?;
        let value = self.get_value_by_key(translation_map, key)?;

        let value_str = if let TranslationValue::String(ref s) = value {
            s
        } else {
            return Err(JsValue::from_str("Translation is not a string"));
        };

        let args_map: HashMap<String, String> = from_value(args)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse arguments: {:?}", e)))?;

        format_string(value_str, &args_map)
            .map_err(|e| JsValue::from_str(&format!("Error during formatting: {:?}", e)))
    }

    fn get_value_by_key<'a>(
        &self,
        map: &'a HashMap<String, TranslationValue>,
        key: &str,
    ) -> Result<&'a TranslationValue, JsValue> {
        let keys: Vec<&str> = key.split('.').collect();
        let mut current = map;

        for k in keys.iter().take(keys.len() - 1) {
            if let Some(TranslationValue::Nested(next_map)) = current.get(*k) {
                current = next_map;
            } else {
                return Err(JsValue::from_str(&format!("Key '{}' not found", k)));
            }
        }

        // Преобразуем последний ключ в `String` для поиска
        current.get(&keys.last().unwrap().to_string()).ok_or_else(|| {
            JsValue::from_str(&format!(
                "Key '{}' not found in the provided translation map",
                keys.last().unwrap()
            ))
        })
    }


    pub fn get_all_translations_for_locale(&self, locale: &str) -> Result<JsValue, JsValue> {
        let translations = self.translations.lock().unwrap();
        let translation = translations.translations.get(locale).ok_or("Locale not found")?;
        to_value(translation).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn get_all_translations(&self) -> Result<JsValue, JsValue> {
        let translations = self.translations.lock().unwrap();
        to_value(&translations.translations).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn has_key_in_translations(&self, locale: &str, key: &str) -> bool {
        let translations = self.translations.lock().unwrap();
        if let Some(translation_map) = translations.translations.get(locale) {
            self.get_value_by_key(translation_map, key).is_ok()
        } else {
            false
        }
    }
}
