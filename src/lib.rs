mod helpers;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use std::collections::HashMap;
use std::sync::RwLock;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};
use crate::helpers::{format_string, get_value_by_key, merge_translation_values, remove_translation_recursive, update_translation_recursive};

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub(crate) enum TranslationValue {
    String(String),
    Nested(HashMap<String, TranslationValue>),
}

#[derive(Serialize, Deserialize)]
struct Translations {
    translations: HashMap<String, HashMap<String, TranslationValue>>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct I18n {
    translations: RwLock<Translations>,
}

#[wasm_bindgen]
impl I18n {
    /// Retrieves all translations for a given locale.
    ///
    /// # Example
    /// ```js
    /// import { I18n } from "wasm-i18n";
    /// let i18n = new I18n();
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new() -> I18n {
        I18n { translations: RwLock::new(Translations {
            translations: HashMap::new(),
        }), }
    }

    /// Retrieves all translations for all locales.
    ///
    /// # Example
    /// ```js
    /// let translations = i18n.translations;
    /// console.log(translations); // { "en": { "hello": "Hello" }, ... }
    /// ```
    #[wasm_bindgen(getter)]
    pub fn translations(&self) -> Result<JsValue, JsValue> {
        let translations = self.translations.read().unwrap();
        serde_wasm_bindgen::to_value(&translations.translations.clone()).map_err(JsValue::from)
    }

    /// Retrieves all available locales.
    ///
    /// # Example
    /// ```js
    /// let locales = i18n.translations;
    /// console.log(locales); // ["en", "fr", "de", ...]
    /// ```
    #[wasm_bindgen(getter)]
    pub fn locales(&self) -> Result<JsValue, JsValue> {
        let translations = self.translations.read().unwrap();
        let values: Vec<String> = translations.translations.keys().cloned().collect();
        serde_wasm_bindgen::to_value(&values).map_err(JsValue::from)
    }

    /// Sets translations for a given locale.
    ///
    /// # Example
    /// ```js
    /// i18n.setTranslations('en', { "hello": "Hello" });
    /// ```
    #[wasm_bindgen(js_name = setTranslations)]
    pub fn set_translations(&self, locale: &str, obj: JsValue) -> Result<(), JsValue> {
        // Преобразуем входной JsValue в HashMap
        let parsed: HashMap<String, TranslationValue> = serde_wasm_bindgen::from_value(obj)?;

        // Получаем доступ к переводам
        let mut translations = self.translations.write().unwrap();

        // Найти или создать место для данного `locale`
        let locale_translations = translations
            .translations
            .entry(locale.to_string())
            .or_default();

        // Рекурсивно объединить переводы
        for (key, value) in parsed {
            locale_translations
                .entry(key)
                .and_modify(|existing_value| merge_translation_values(existing_value, value.clone()))
                .or_insert(value);
        }

        Ok(())
    }



    /// Gets a translation for a given key and locale.
    ///
    /// # Example
    /// ```js
    /// let translation = i18n.getTranslation('en', 'hello');
    /// console.log(translation); // "Hello"
    /// ```
    #[wasm_bindgen(js_name = getTranslation)]
    pub fn get_translation(&self, locale: &str, key: &str) -> Result<JsValue, JsValue> {
        let translations = self.translations.read().unwrap();
        let value = get_value_by_key(
            translations
                .translations
                .get(locale)
                .ok_or("Locale not found")?,
            key,
        )
            .cloned()
            .map_err(JsValue::from)?;
        serde_wasm_bindgen::to_value(&value).map_err(JsValue::from)
    }


    /// Checks if a translation exists for a given key and locale.
    ///
    /// # Example
    /// ```js
    /// let exists = i18n.hasTranslation('en', 'hello');
    /// console.log(exists); // true or false
    /// ```
    #[wasm_bindgen(js_name = hasTranslation)]
    pub fn has_translation(&self, locale: &str, key: &str) -> bool {
        let translations = self.translations.read().unwrap();
        if let Some(translation_map) = translations.translations.get(locale) {
            get_value_by_key(translation_map, key).is_ok()
        } else {
            false
        }
    }

    /// Deletes a translation for a given key and locale.
    ///
    /// # Example
    /// ```js
    /// await i18n.delTranslation('en', 'hello');
    /// ```
    #[wasm_bindgen(js_name = delTranslation)]
    pub fn del_translation(&self, locale: &str, key: &str) -> Result<(), JsValue> {
        let mut translations = self.translations.write().unwrap();
        let keys: Vec<&str> = key.split('.').collect();

        let val = translations
            .translations
            .get_mut(locale)
            .ok_or("Locale not found")?;
        remove_translation_recursive(val, &keys)?;

        Ok(())
    }


    /// Retrieves all translations for a given locale.
    ///
    /// # Example
    /// ```js
    /// let translations = i18n.getTranslations('en');
    /// console.log(translations); // { "hello": "Hello", ... }
    /// ```
    #[wasm_bindgen(js_name = getTranslations)]
    pub fn get_translations(&self, locale: &str) -> Result<JsValue, JsValue> {
        let translations = self.translations.read().unwrap();
        let values = translations
            .translations
            .get(locale)
            .cloned()
            .ok_or("Locale not found".to_string())
            .map_err(JsValue::from)?;
        serde_wasm_bindgen::to_value(&values).map_err(JsValue::from)
    }

    /// Deletes all translations for a given locale.
    ///
    /// # Example
    /// ```js
    /// i18n.delTranslations('en');
    /// ```
    #[wasm_bindgen(js_name = delTranslations)]
    pub fn del_translations(&self, locale: &str) -> Result<(), JsValue> {
        let mut translations = self.translations.write().unwrap();
        translations.translations.remove(locale);
        Ok(())
    }

    /// Checks if a given locale exists.
    ///
    /// # Example
    /// ```js
    /// let exists = i18n.hasLocale('en');
    /// console.log(exists); // true or false
    /// ```
    #[wasm_bindgen(js_name = hasLocale)]
    pub fn has_locale(&self, locale: &str) -> bool {
        let translations = self.translations.read().unwrap();
        translations.translations.contains_key(locale)
    }

    /// Clears all translations for all locales.
    ///
    /// # Example
    /// ```js
    /// i18n.clearAllTranslations();
    /// ```
    #[wasm_bindgen(js_name = clearAllTranslations)]
    pub fn clear_all_translations(&self) -> Result<(), JsValue> {
        let mut translations = self.translations.write().unwrap();
        translations.translations.clear();
        Ok(())
    }

    /// Loads translations from a remote URL and updates the translation manager.
    ///
    /// # Example
    /// ```js
    /// await i18n.loadTranslations('https://example.com/translations.json');
    /// ```
    #[wasm_bindgen(js_name = loadTranslations)]
    pub async fn load_translations(&self, url: &str) -> Result<(), JsValue> {
        let opts = RequestInit::new();
        opts.set_method("GET");

        let request = Request::new_with_str_and_init(url, &opts)?;
        let window = web_sys::window().ok_or("Window not found")?;
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into()?;
        let json = JsFuture::from(resp.json()?).await?;
        let translations: HashMap<String, HashMap<String, TranslationValue>> =
            serde_wasm_bindgen::from_value(json)?;

        for (locale, translation) in translations {
            for (key, value) in translation {
                let mut translations = self.translations.write().unwrap();
                let translation_map = translations.translations
                    .get_mut(&locale)
                    .ok_or("Locale not found")?;

                let keys: Vec<&str> = key.split('.').collect();
                update_translation_recursive(translation_map, &keys, value)?;
            }
        }

        Ok(())
    }

    /// Updates a translation for a given locale and key.
    ///
    /// # Example
    /// ```js
    /// i18n.updateTranslation('en', 'hello', 'Hi');
    /// ```
    #[wasm_bindgen(js_name = updateTranslation)]
    pub fn update_translation(&self, locale: &str, key: &str, value: JsValue) -> Result<(), JsValue> {
        let parsed_value: TranslationValue = serde_wasm_bindgen::from_value(value)?;

        let mut translations = self.translations.write().unwrap();
        let translation_map = translations
            .translations
            .get_mut(locale)
            .ok_or("Locale not found")?;

        let keys: Vec<&str> = key.split('.').collect();
        update_translation_recursive(translation_map, &keys, parsed_value)
    }

    /// Formats a translation for a given locale, key, and arguments.
    ///
    /// # Example
    /// ```js
    /// let formatted = i18n.format_translation('en', 'greeting', { name: 'Alice' });
    /// console.log(formatted); // "Hello, Alice!"
    /// ```
    #[wasm_bindgen(js_name = formatTranslation)]
    pub fn format_translation(&self, locale: &str, key: &str, args: JsValue) -> Result<String, JsValue> {
        let translations = self.translations.read().unwrap();
        let value = get_value_by_key(
            translations
                .translations
                .get(locale)
                .ok_or("Locale not found")?,
            key,
        )?;

        let args_map: HashMap<String, String> = serde_wasm_bindgen::from_value(args)?;
        match value {
            TranslationValue::String(s) => format_string(s, &args_map).map_err(JsValue::from),
            TranslationValue::Nested(_) => {
                Err(JsValue::from("Translation is not a string".to_string()))
            }
        }
    }

    /// Retrieves all translations for a specific locale.
    ///
    /// # Example
    /// ```js
    /// let translations = i18n.getAlTranslationsForLocale('en');
    /// console.log(translations); // { "hello": "Hello", ... }
    /// ```
    #[wasm_bindgen(js_name = getAlTranslationsForLocale)]
    pub fn get_all_translations_for_locale(&self, locale: &str) -> Result<JsValue, JsValue> {
        let translations = self.translations.read().unwrap();
        let values = translations
            .translations
            .get(locale)
            .cloned()
            .ok_or("Locale not found".to_string())
            .map_err(JsValue::from)?;
        serde_wasm_bindgen::to_value(&values).map_err(JsValue::from)
    }

    /// Checks if a translation key exists in any locale's translations.
    ///
    /// # Example
    /// ```js
    /// let exists = i18n.hasKeyInTranslations('en', 'hello');
    /// console.log(exists); // true or false
    /// ```
    #[wasm_bindgen(js_name = hasKeyInTranslations)]
    pub fn has_key_in_translations(&self, locale: &str, key: &str) -> bool {
        let translations = self.translations.read().unwrap();
        if let Some(translation_map) = translations.translations.get(locale) {
            get_value_by_key(translation_map, key).is_ok()
        } else {
            false
        }
    }
}