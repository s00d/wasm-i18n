use crate::format::format_string;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;

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
    translations: RwLock<Translations>,
}

impl TranslationManager {
    /// Creates a new instance of TranslationManager
    pub fn new() -> Self {
        Self {
            translations: RwLock::new(Translations {
                translations: HashMap::new(),
            }),
        }
    }

    /// Adds or updates translations for a given locale
    pub fn set_translations(&self, locale: &str, json: &str) -> Result<(), String> {
        let parsed: HashMap<String, TranslationValue> =
            serde_json::from_str(json).map_err(|e| e.to_string())?;

        let mut translations = self.translations.write().unwrap();
        translations
            .translations
            .entry(locale.to_string())
            .or_default()
            .extend(parsed);
        Ok(())
    }

    /// Retrieves a translation by locale and key
    pub fn get_translation(&self, locale: &str, key: &str) -> Result<TranslationValue, String> {
        let translations = self.translations.read().unwrap();
        self.get_value_by_key(
            translations
                .translations
                .get(locale)
                .ok_or("Locale not found")?,
            key,
        )
        .cloned()
    }

    /// Checks if a specific key exists in a locale
    pub fn has_translation(&self, locale: &str, key: &str) -> bool {
        let translations = self.translations.read().unwrap();
        if let Some(translation_map) = translations.translations.get(locale) {
            self.get_value_by_key(translation_map, key).is_ok()
        } else {
            false
        }
    }

    /// Retrieves all translations for a locale
    pub fn get_translations(
        &self,
        locale: &str,
    ) -> Result<HashMap<String, TranslationValue>, String> {
        let translations = self.translations.read().unwrap();
        translations
            .translations
            .get(locale)
            .cloned()
            .ok_or("Locale not found".to_string())
    }

    /// Deletes all translations for a specific locale
    pub fn del_translations(&self, locale: &str) -> Result<(), String> {
        let mut translations = self.translations.write().unwrap();
        translations.translations.remove(locale);
        Ok(())
    }

    /// Updates a specific translation
    pub fn update_translation(
        &self,
        locale: &str,
        key: &str,
        value: TranslationValue,
    ) -> Result<(), String> {
        let mut translations = self.translations.write().unwrap();
        let translation_map = translations
            .translations
            .get_mut(locale)
            .ok_or("Locale not found")?;
        self.update_nested_value(translation_map, key, value)
    }

    /// Deletes a specific translation by key
    pub fn del_translation(&self, locale: &str, key: &str) -> Result<(), String> {
        let mut translations = self.translations.write().unwrap();
        if let Some(translation_map) = translations.translations.get_mut(locale) {
            self.delete_nested_value(translation_map, key);
        }
        Ok(())
    }

    /// Formats a translation with provided arguments
    pub fn format_translation(
        &self,
        locale: &str,
        key: &str,
        args: HashMap<String, String>,
    ) -> Result<String, String> {
        let translations = self.translations.read().unwrap();
        let value = self.get_value_by_key(
            translations
                .translations
                .get(locale)
                .ok_or("Locale not found")?,
            key,
        )?;

        match value {
            TranslationValue::String(s) => {
                format_string(s, &args).map_err(|e| format!("Error during formatting: {:?}", e))
            }
            _ => Err("Translation is not a string".to_string()),
        }
    }

    /// Retrieves all available locales
    pub fn get_all_locales(&self) -> Vec<String> {
        let translations = self.translations.read().unwrap();
        translations.translations.keys().cloned().collect()
    }

    /// Checks if a locale exists
    pub fn has_locale(&self, locale: &str) -> bool {
        let translations = self.translations.read().unwrap();
        translations.translations.contains_key(locale)
    }

    /// Clears all translations
    pub fn clear_all_translations(&self) -> Result<(), String> {
        let mut translations = self.translations.write().unwrap();
        translations.translations.clear();
        Ok(())
    }

    // ======== PRIVATE HELPERS ========

    pub fn get_all_translations_for_locale(
        &self,
        locale: &str,
    ) -> Result<HashMap<String, TranslationValue>, String> {
        let translations = self.translations.read().unwrap();
        translations
            .translations
            .get(locale)
            .cloned()
            .ok_or("Locale not found".to_string())
    }

    pub fn get_all_translations(&self) -> HashMap<String, HashMap<String, TranslationValue>> {
        let translations = self.translations.read().unwrap();
        translations.translations.clone()
    }

    pub fn has_key_in_translations(&self, locale: &str, key: &str) -> bool {
        let translations = self.translations.read().unwrap();
        if let Some(translation_map) = translations.translations.get(locale) {
            self.get_value_by_key(translation_map, key).is_ok()
        } else {
            false
        }
    }

    /// Retrieves a value by its nested key
    fn get_value_by_key<'a>(
        &self,
        map: &'a HashMap<String, TranslationValue>,
        key: &str,
    ) -> Result<&'a TranslationValue, String> {
        let keys: Vec<&str> = key.split('.').collect();
        let mut current = map;

        for k in &keys[..keys.len() - 1] {
            if let Some(TranslationValue::Nested(next_map)) = current.get(*k) {
                current = next_map;
            } else {
                return Err(format!("Key '{}' not found", k));
            }
        }

        current
            .get(&keys.last().unwrap().to_string())
            .ok_or_else(|| {
                format!(
                    "Key '{}' not found in the provided translation map",
                    keys.last().unwrap()
                )
            })
    }

    /// Updates a nested value by key
    fn update_nested_value(
        &self,
        map: &mut HashMap<String, TranslationValue>,
        key: &str,
        value: TranslationValue,
    ) -> Result<(), String> {
        let keys: Vec<&str> = key.split('.').collect();
        let mut current = map;

        for k in &keys[..keys.len() - 1] {
            current = current
                .entry(k.to_string())
                .or_insert_with(|| TranslationValue::Nested(HashMap::new()))
                .as_nested_mut()
                .ok_or("Invalid key path")?;
        }

        current.insert(keys.last().unwrap().to_string(), value);
        Ok(())
    }

    /// Deletes a nested value by key
    fn delete_nested_value(&self, map: &mut HashMap<String, TranslationValue>, key: &str) {
        let keys: Vec<&str> = key.split('.').collect();
        let mut current = map;

        for k in &keys[..keys.len() - 1] {
            if let Some(TranslationValue::Nested(next_map)) = current.get_mut(*k) {
                current = next_map;
            } else {
                return; // If path doesn't exist, do nothing
            }
        }

        if let Some(last_key) = keys.last() {
            current.remove(*last_key);
        }
    }
}

impl TranslationValue {
    fn as_nested_mut(&mut self) -> Option<&mut HashMap<String, TranslationValue>> {
        if let TranslationValue::Nested(map) = self {
            Some(map)
        } else {
            None
        }
    }
}
