#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod format;
mod manager;

use std::collections::HashMap;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::{JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::manager::TranslationManager;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

lazy_static::lazy_static! {
    static ref TRANSLATION_MANAGER: TranslationManager = TranslationManager::new();
}

#[wasm_bindgen]
pub fn set_translations(locale: &str, json: &str) -> Result<(), JsValue> {
    TRANSLATION_MANAGER.set_translations(locale, json)
}

#[wasm_bindgen]
pub fn get_translation(locale: &str, key: &str) -> Result<JsValue, JsValue> {
    TRANSLATION_MANAGER.get_translation(locale, key)
}

#[wasm_bindgen]
pub fn has_translation(locale: &str, key: &str) -> bool {
    TRANSLATION_MANAGER.has_translation(locale, key)
}

#[wasm_bindgen]
pub fn del_translation(locale: &str, key: &str) -> Result<(), JsValue> {
    TRANSLATION_MANAGER.del_translation(locale, key)
}

#[wasm_bindgen]
pub fn set_translations_from_object(locale: &str, obj: JsValue) -> Result<(), JsValue> {
    let parsed: HashMap<String, manager::TranslationValue> = from_value(obj)?;
    let json_str = serde_json::to_string(&parsed).map_err(|e| JsValue::from_str(&e.to_string()))?;
    TRANSLATION_MANAGER.set_translations(locale, &json_str)
}

#[wasm_bindgen]
pub fn get_translations(locale: &str) -> Result<JsValue, JsValue> {
    TRANSLATION_MANAGER.get_translations(locale)
}

#[wasm_bindgen]
pub fn del_translations(locale: &str) -> Result<(), JsValue> {
    TRANSLATION_MANAGER.del_translations(locale)
}

#[wasm_bindgen]
pub fn has_locale(locale: &str) -> bool {
    TRANSLATION_MANAGER.has_locale(locale)
}

#[wasm_bindgen]
pub fn clear_all_translations() -> Result<(), JsValue> {
    TRANSLATION_MANAGER.clear_all_translations()
}

#[wasm_bindgen]
pub async fn load_translations(url: &str) -> Result<(), JsValue> {
    TRANSLATION_MANAGER.load_translations(url).await
}

#[wasm_bindgen]
pub fn get_all_locales() -> Result<JsValue, JsValue> {
    TRANSLATION_MANAGER.get_all_locales()
}

#[wasm_bindgen]
pub fn update_translation(locale: &str, key: &str, value: JsValue) -> Result<(), JsValue> {
    TRANSLATION_MANAGER.update_translation(locale, key, value)
}

#[wasm_bindgen]
pub fn format_translation(locale: &str, key: &str, args: JsValue) -> Result<String, JsValue> {
    TRANSLATION_MANAGER.format_translation(locale, key, args)
}

#[wasm_bindgen]
pub fn get_all_translations_for_locale(locale: &str) -> Result<JsValue, JsValue> {
    TRANSLATION_MANAGER.get_all_translations_for_locale(locale)
}

#[wasm_bindgen]
pub fn get_all_translations() -> Result<JsValue, JsValue> {
    TRANSLATION_MANAGER.get_all_translations()
}

#[wasm_bindgen]
pub fn has_key_in_translations(locale: &str, key: &str) -> bool {
    TRANSLATION_MANAGER.has_key_in_translations(locale, key)
}



