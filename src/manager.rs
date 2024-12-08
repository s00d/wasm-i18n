use serde::{Deserialize, Serialize};
use dashmap::DashMap;

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub(crate) enum TranslationValue {
    String(String),
    Nested(DashMap<String, TranslationValue>),
}

#[derive(Serialize, Deserialize)]
struct Translations {
    translations: DashMap<String, DashMap<String, TranslationValue>>,
}

pub struct TranslationManager {
    translations: Translations,
}

impl TranslationManager {
    /// Creates a new instance of TranslationManager
    pub fn new() -> Self {
        Self {
            translations: Translations {
                translations: DashMap::new(),
            },
        }
    }

    /// Adds or updates translations for a given locale
    pub fn set_translations(&self, locale: &str, json: DashMap<String, TranslationValue>) -> Result<(), String> {
        self.translations
            .translations
            .entry(locale.to_string())
            .or_default()
            .extend(json);
        Ok(())
    }

    /// Retrieves a translation by locale and key
    pub fn get_translation(&self, locale: &str, key: &str) -> Result<TranslationValue, String> {
        self.get_value_by_key(
            self.translations
                .translations
                .get(locale)
                .ok_or("Locale not found")?
                .value(),
            key,
        )
    }

    /// Checks if a specific key exists in a locale
    pub fn has_translation(&self, locale: &str, key: &str) -> bool {
        if let Some(translation_map) = self.translations.translations.get(locale) {
            self.get_value_by_key(translation_map.value(), key).is_ok()
        } else {
            false
        }
    }

    /// Retrieves all translations for a locale
    pub fn get_translations(
        &self,
        locale: &str,
    ) -> Result<DashMap<String, TranslationValue>, String> {
        self.translations
            .translations
            .get(locale)
            .map(|map| map.clone())  // Clone the DashMap to return owned version
            .ok_or("Locale not found".to_string())
    }

    /// Deletes all translations for a specific locale
    pub fn del_translations(&self, locale: &str) -> Result<(), String> {
        self.translations.translations.remove(locale);
        Ok(())
    }

    /// Updates a specific translation
    pub fn update_translation(
        &self,
        locale: &str,
        key: &str,
        value: TranslationValue,
    ) -> Result<(), String> {
        let mut translation_map = self.translations
            .translations
            .get_mut(locale)
            .ok_or("Locale not found")?;

        let keys: Vec<&str> = key.split('.').collect();
        self.update_translation_recursive(&mut translation_map, &keys, value)
    }

    /// Deletes a specific translation by key
    pub fn del_translation(&self, locale: &str, key: &str) -> Result<(), String> {
        let mut translation_map = self.translations
            .translations
            .get_mut(locale)
            .ok_or("Locale not found")?;

        let keys: Vec<&str> = key.split('.').collect();
        self.remove_translation_recursive(&mut translation_map, &keys)?;

        Ok(())
    }

    /// Formats a translation with provided arguments
    pub fn format_translation(
        &self,
        locale: &str,
        key: &str,
        args: DashMap<String, String>,
    ) -> Result<String, String> {
        let value = self.get_value_by_key(
            self.translations
                .translations
                .get(locale)
                .ok_or("Locale not found")?
                .value(),
            key,
        )?;

        match value {
            TranslationValue::String(s) => {
                self.format_string(s, args).map_err(|e| format!("Error during formatting: {:?}", e))
            }
            _ => Err("Translation is not a string".to_string()),
        }
    }

    pub fn format_string(&self, template: String, args: DashMap<String, String>) -> Result<String, String> {
        let mut result = template.to_string();
        for (key, value) in args {
            let placeholder = format!("{{{}}}", key);
            result = result.replace(&placeholder, &*value);
        }
        Ok(result)
    }

    /// Retrieves all available locales
    pub fn get_all_locales(&self) -> Vec<String> {
        self.translations.translations
            .iter()
            .map(|entry| entry.key().clone())  // Получаем ключ (String) и клонируем его
            .collect()
    }

    /// Checks if a locale exists
    pub fn has_locale(&self, locale: &str) -> bool {
        self.translations.translations.contains_key(locale)
    }

    /// Clears all translations
    pub fn clear_all_translations(&self) -> Result<(), String> {
        self.translations.translations.clear();
        Ok(())
    }

    pub fn get_all_translations_for_locale(
        &self,
        locale: &str,
    ) -> Result<DashMap<String, TranslationValue>, String> {
        self.translations
            .translations
            .get(locale)
            .map(|map| map.to_owned())  // Клонируем DashMap
            .ok_or("Locale not found".to_string())
    }

    pub fn get_all_translations(&self) -> DashMap<String, DashMap<String, TranslationValue>> {
        self.translations.translations.clone()
    }

    pub fn has_key_in_translations(&self, locale: &str, key: &str) -> bool {
        if let Some(translation_map) = self.translations.translations.get(locale) {
            self.get_value_by_key(translation_map.value(), key).is_ok()
        } else {
            false
        }
    }


    // ======== PRIVATE HELPERS ========


    /// Retrieves a value by its nested key
    fn get_value_by_key(
        &self,
        map: &DashMap<String, TranslationValue>,
        key: &str,
    ) -> Result<TranslationValue, String> {  // Return owned value, not a reference
        let keys: Vec<&str> = key.split('.').collect();
        let mut current = map.clone();  // Clone the entire map, so we have ownership

        for k in &keys[..keys.len() - 1] {
            if let Some(TranslationValue::Nested(next_map)) = current.get(*k).map(|v| v.clone()) {  // Clone the value here
                current = next_map;  // Move the cloned map into current
            } else {
                return Err(format!("Key '{}' not found", k));
            }
        }

        let val = current
            .get(&keys.last().unwrap().to_string())
            .ok_or_else(|| {
                format!(
                    "Key '{}' not found in the provided translation map",
                    keys.last().unwrap()
                )
            })?.clone();  // Clone the value here

        Ok(val)
    }

    fn update_translation_recursive(
        &self,
        current_map: &mut DashMap<String, TranslationValue>,
        keys: &[&str],
        value: TranslationValue,
    ) -> Result<(), String> {
        if keys.len() == 1 {
            // Base case: if there's only one key left, insert the value
            current_map.insert(keys[0].to_string(), value);
            Ok(())
        } else {
            // Recursive case: traverse deeper into the nested structure
            let next_key = keys[0];
            let mut entry = current_map
                .entry(next_key.to_string())
                .or_insert_with(|| TranslationValue::Nested(DashMap::new()));

            // Check that the entry is not a string (i.e., it's a Nested variant)
            match entry.value_mut() {
                TranslationValue::String(_) => {
                    Err("Invalid key path: found a string where a nested map was expected".to_string())
                }
                TranslationValue::Nested(ref mut nested_map) => {
                    // Recursively call for the next level
                    self.update_translation_recursive(nested_map, &keys[1..], value)
                }
            }
        }
    }


    /// Deletes a nested value by key
    fn remove_translation_recursive(
        &self,
        current_map: &mut DashMap<String, TranslationValue>,
        keys: &[&str],
    ) -> Result<(), String> {
        if keys.len() == 1 {
            // Base case: remove the key at the current level
            current_map.remove(keys[0]);
            Ok(())
        } else {
            // Recursive case: traverse deeper into the nested structure
            let next_key = keys[0];
            let mut entry = current_map
                .entry(next_key.to_string())
                .or_insert_with(|| TranslationValue::Nested(DashMap::new()));
            match entry.value_mut() {
                TranslationValue::String(_) => {
                    Err("Invalid key path: found a string where a nested map was expected".to_string())
                }
                TranslationValue::Nested(ref mut nested_map) => {
                    // Recursively call for the next level
                    self.remove_translation_recursive(nested_map, &keys[1..])?;
                    Ok(())
                }
            }
        }
    }
}

// impl TranslationValue {
//     fn as_nested_mut(&mut self) -> Option<&mut DashMap<String, TranslationValue>> {
//         if let TranslationValue::Nested(map) = self {
//             Some(map)
//         } else {
//             None
//         }
//     }
// }
