#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod manager;

use crate::manager::{TranslationManager, TranslationValue};
use std::collections::HashMap;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct I18n {
    manager: TranslationManager
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
        I18n { manager: TranslationManager::new() }
    }

    /// Returns the version of the project at compile-time.
    ///
    /// # Example
    /// ```js
    /// let version = i18n.version;
    /// console.log(version); // "1.0.0"
    /// ```
    #[wasm_bindgen(getter)]
    pub fn version() -> String {
        let version = env!("CARGO_PKG_VERSION");
        version.to_string()
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
        let translations = self.manager.get_all_translations();
        serde_wasm_bindgen::to_value(&translations).map_err(JsValue::from)
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
        let locales = self.manager.get_all_locales();
        serde_wasm_bindgen::to_value(&locales).map_err(JsValue::from)
    }

    /// Sets translations for a given locale.
    ///
    /// # Example
    /// ```js
    /// i18n.setTranslations('en', { "hello": "Hello" });
    /// ```
    #[wasm_bindgen(js_name = setTranslations)]
    pub fn set_translations(&self, locale: &str, obj: JsValue) -> Result<(), JsValue> {
        let parsed: HashMap<String, TranslationValue> = serde_wasm_bindgen::from_value(obj)?;
        self.manager.set_translations(locale, parsed)
            .map_err(JsValue::from)
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
        let value = self.manager
            .get_translation(locale, key)
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
        self.manager.has_translation(locale, key)
    }

    /// Deletes a translation for a given key and locale.
    ///
    /// # Example
    /// ```js
    /// await i18n.delTranslation('en', 'hello');
    /// ```
    #[wasm_bindgen(js_name = delTranslation)]
    pub fn del_translation(&self, locale: &str, key: &str) -> Result<(), JsValue> {
        self.manager
            .del_translation(locale, key)
            .map_err(JsValue::from)
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
        let translations = self.manager
            .get_translations(locale)
            .map_err(JsValue::from)?;
        serde_wasm_bindgen::to_value(&translations).map_err(JsValue::from)
    }

    /// Deletes all translations for a given locale.
    ///
    /// # Example
    /// ```js
    /// i18n.delTranslations('en');
    /// ```
    #[wasm_bindgen(js_name = delTranslations)]
    pub fn del_translations(&self, locale: &str) -> Result<(), JsValue> {
        self.manager
            .del_translations(locale)
            .map_err(JsValue::from)
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
        self.manager.has_locale(locale)
    }

    /// Clears all translations for all locales.
    ///
    /// # Example
    /// ```js
    /// i18n.clearAllTranslations();
    /// ```
    #[wasm_bindgen(js_name = clearAllTranslations)]
    pub fn clear_all_translations(&self) -> Result<(), JsValue> {
        self.manager
            .clear_all_translations()
            .map_err(JsValue::from)
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
                self.manager
                    .update_translation(&locale, &key, value)
                    .map_err(JsValue::from)?;
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
        self.manager
            .update_translation(locale, key, parsed_value)
            .map_err(JsValue::from)
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
        let args_map: HashMap<String, String> = serde_wasm_bindgen::from_value(args)?;
        self.manager
            .format_translation(locale, key, args_map)
            .map_err(JsValue::from)
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
        let translations = self.manager
            .get_all_translations_for_locale(locale)
            .map_err(JsValue::from)?;
        serde_wasm_bindgen::to_value(&translations).map_err(JsValue::from)
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
        self.manager.has_key_in_translations(locale, key)
    }
}