#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod format;
mod manager;

use std::collections::HashMap;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};
use crate::manager::{TranslationManager, TranslationValue};

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
    TRANSLATION_MANAGER.set_translations(locale, json).map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen]
pub fn get_translation(locale: &str, key: &str) -> Result<JsValue, JsValue> {
    let value = TRANSLATION_MANAGER.get_translation(locale, key).map_err(|e| JsValue::from_str(&e))?;
    serde_wasm_bindgen::to_value(&value).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn has_translation(locale: &str, key: &str) -> bool {
    TRANSLATION_MANAGER.has_translation(locale, key)
}

#[wasm_bindgen]
pub fn del_translation(locale: &str, key: &str) -> Result<(), JsValue> {
    TRANSLATION_MANAGER.del_translation(locale, key).map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen]
pub fn set_translations_from_object(locale: &str, obj: JsValue) -> Result<(), JsValue> {
    let parsed: HashMap<String, TranslationValue> = serde_wasm_bindgen::from_value(obj)?;
    let json_str = serde_json::to_string(&parsed).map_err(|e| JsValue::from_str(&e.to_string()))?;
    TRANSLATION_MANAGER.set_translations(locale, &json_str).map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen]
pub fn get_translations(locale: &str) -> Result<JsValue, JsValue> {
    let translations = TRANSLATION_MANAGER.get_translations(locale).map_err(|e| JsValue::from_str(&e))?;
    serde_wasm_bindgen::to_value(&translations).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn del_translations(locale: &str) -> Result<(), JsValue> {
    TRANSLATION_MANAGER.del_translations(locale).map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen]
pub fn has_locale(locale: &str) -> bool {
    TRANSLATION_MANAGER.has_locale(locale)
}

#[wasm_bindgen]
pub fn clear_all_translations() -> Result<(), JsValue> {
    TRANSLATION_MANAGER.clear_all_translations().map_err(|e| JsValue::from_str(&e))
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
    let translations: HashMap<String, HashMap<String, TranslationValue>> = serde_wasm_bindgen::from_value(json)?;

    for (locale, translation) in translations {
        for (key, value) in translation {
            TRANSLATION_MANAGER.update_translation(&locale, &key, value).map_err(|e| JsValue::from_str(&e))?;
        }
    }

    Ok(())
}

#[wasm_bindgen]
pub fn get_all_locales() -> Result<JsValue, JsValue> {
    let locales = TRANSLATION_MANAGER.get_all_locales();
    serde_wasm_bindgen::to_value(&locales).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn update_translation(locale: &str, key: &str, value: JsValue) -> Result<(), JsValue> {
    let parsed_value: TranslationValue = serde_wasm_bindgen::from_value(value)?;
    TRANSLATION_MANAGER.update_translation(locale, key, parsed_value).map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen]
pub fn format_translation(locale: &str, key: &str, args: JsValue) -> Result<String, JsValue> {
    let args_map: HashMap<String, String> = serde_wasm_bindgen::from_value(args)?;
    TRANSLATION_MANAGER.format_translation(locale, key, args_map).map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen]
pub fn get_all_translations_for_locale(locale: &str) -> Result<JsValue, JsValue> {
    let translations = TRANSLATION_MANAGER.get_all_translations_for_locale(locale).map_err(|e| JsValue::from_str(&e))?;
    serde_wasm_bindgen::to_value(&translations).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn get_all_translations() -> Result<JsValue, JsValue> {
    let translations = TRANSLATION_MANAGER.get_all_translations();
    serde_wasm_bindgen::to_value(&translations).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn has_key_in_translations(locale: &str, key: &str) -> bool {
    TRANSLATION_MANAGER.has_key_in_translations(locale, key)
}
