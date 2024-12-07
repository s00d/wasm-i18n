use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
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

    pub fn set_translations(&self, locale: &str, json: &str) -> Result<(), String> {
        let mut translations = self.translations.lock().unwrap();
        let parsed: HashMap<String, TranslationValue> =
            serde_json::from_str(json).map_err(|e| e.to_string())?;

        translations
            .translations
            .entry(locale.to_string())
            .or_default()
            .extend(parsed);
        Ok(())
    }

    pub fn get_translation(&self, locale: &str, key: &str) -> Result<TranslationValue, String> {
        let translations = self.translations.lock().unwrap();
        let translation_map = translations
            .translations
            .get(locale)
            .ok_or_else(|| "Locale not found".to_string())?;
        self.get_value_by_key(translation_map, key).cloned()
    }

    pub fn get_translations(&self, locale: &str) -> Result<HashMap<String, TranslationValue>, String> {
        let translations = self.translations.lock().unwrap();
        translations.translations.get(locale).cloned().ok_or("Locale not found".to_string())
    }

    pub fn del_translations(&self, locale: &str) -> Result<(), String> {
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

    pub fn get_all_locales(&self) -> Vec<String> {
        let translations = self.translations.lock().unwrap();
        translations.translations.keys().cloned().collect()
    }

    pub fn update_translation(&self, locale: &str, key: &str, value: TranslationValue) -> Result<(), String> {
        let mut translations = self.translations.lock().unwrap();
        if let Some(existing) = translations.translations.get_mut(locale) {
            let keys: Vec<&str> = key.split('.').collect();
            let mut current = existing;
            for k in keys.iter().take(keys.len() - 1) {
                current = match current.get_mut(*k) {
                    Some(TranslationValue::Nested(next_map)) => next_map,
                    _ => return Err("Invalid key path".to_string()),
                };
            }
            if let Some(last_key) = keys.last() {
                current.insert(last_key.to_string(), value);
            }
        } else {
            return Err("Locale not found".to_string());
        }
        Ok(())
    }

    pub fn del_translation(&self, locale: &str, key: &str) -> Result<(), String> {
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

    pub fn clear_all_translations(&self) -> Result<(), String> {
        let mut translations = self.translations.lock().unwrap();
        translations.translations.clear();
        Ok(())
    }

    pub fn format_translation(&self, locale: &str, key: &str, args: HashMap<String, String>) -> Result<String, String> {
        let translations = self.translations.lock().unwrap();
        let translation_map = translations
            .translations
            .get(locale)
            .ok_or_else(|| "Locale not found".to_string())?;
        let value = self.get_value_by_key(translation_map, key)?;

        let value_str = if let TranslationValue::String(ref s) = value {
            s
        } else {
            return Err("Translation is not a string".to_string());
        };

        format_string(value_str, &args)
            .map_err(|e| format!("Error during formatting: {:?}", e))
    }

    fn get_value_by_key<'a>(
        &self,
        map: &'a HashMap<String, TranslationValue>,
        key: &str,
    ) -> Result<&'a TranslationValue, String> {
        let keys: Vec<&str> = key.split('.').collect();
        let mut current = map;

        for k in keys.iter().take(keys.len() - 1) {
            if let Some(TranslationValue::Nested(next_map)) = current.get(*k) {
                current = next_map;
            } else {
                return Err(format!("Key '{}' not found", k));
            }
        }

        // Преобразуем последний ключ в `String` для поиска
        current.get(&keys.last().unwrap().to_string()).ok_or_else(|| {
            format!(
                "Key '{}' not found in the provided translation map",
                keys.last().unwrap()
            )
        })
    }

    pub fn get_all_translations_for_locale(&self, locale: &str) -> Result<HashMap<String, TranslationValue>, String> {
        let translations = self.translations.lock().unwrap();
        translations.translations.get(locale).cloned().ok_or("Locale not found".to_string())
    }

    pub fn get_all_translations(&self) -> HashMap<String, HashMap<String, TranslationValue>> {
        let translations = self.translations.lock().unwrap();
        translations.translations.clone()
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
