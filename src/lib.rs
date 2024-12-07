#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod format;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use std::collections::HashMap;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};
use wasm_bindgen::prelude::wasm_bindgen;
use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::format::format_string;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
enum TranslationValue {
    String(String),
    Nested(HashMap<String, TranslationValue>),
}

#[derive(Serialize, Deserialize, Debug)]
struct Translations {
    translations: HashMap<String, HashMap<String, TranslationValue>>,
}

lazy_static! {
    static ref TRANSLATIONS: Mutex<Translations> = Mutex::new(Translations {
        translations: HashMap::new(),
    });
}

#[wasm_bindgen]
pub fn set_translations(locale: &str, json: &str) -> Result<(), JsValue> {
    let mut translations = TRANSLATIONS.lock().unwrap();
    let parsed: HashMap<String, TranslationValue> = serde_json::from_str(json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    translations
        .translations
        .entry(locale.to_string())
        .or_default()
        .extend(parsed);
    Ok(())
}

#[wasm_bindgen]
pub fn get_translation(locale: &str, key: &str) -> Result<JsValue, JsValue> {
    let translations = TRANSLATIONS.lock().unwrap();
    let translation_map = translations.translations.get(locale).ok_or("Locale not found")?;
    let value = get_value_by_key(translation_map, key)?;

    // Конвертируем найденное значение в JsValue для возврата строк или объектов
    match value {
        TranslationValue::String(s) => Ok(JsValue::from_str(s)),
        TranslationValue::Nested(nested) => to_value(nested).map_err(|e| JsValue::from_str(&e.to_string())),
    }
}

fn get_value_by_key<'a>(
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
    current.get(&keys.last().unwrap().to_string()).ok_or_else(|| {
        JsValue::from_str(&format!(
            "Key '{}' not found in the provided translation map",
            keys.last().unwrap()
        ))
    })
}

#[wasm_bindgen]
pub fn has_translation(locale: &str, key: &str) -> bool {
    let translations = TRANSLATIONS.lock().unwrap();
    if let Some(translation_map) = translations.translations.get(locale) {
        get_value_by_key(translation_map, key).is_ok()
    } else {
        false
    }
}

#[wasm_bindgen]
pub fn del_translation(locale: &str, key: &str) -> Result<(), JsValue> {
    let mut translations = TRANSLATIONS.lock().unwrap();
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

#[wasm_bindgen]
pub fn set_translations_from_object(locale: &str, obj: JsValue) -> Result<(), JsValue> {
    let parsed: HashMap<String, TranslationValue> = from_value(obj)?;
    let json_str = serde_json::to_string(&parsed).map_err(|e| JsValue::from_str(&e.to_string()))?;
    set_translations(locale, &json_str)
}

#[wasm_bindgen]
pub fn get_translations(locale: &str) -> Result<JsValue, JsValue> {
    let translations = TRANSLATIONS.lock().unwrap();
    let translation = translations.translations.get(locale).ok_or("Locale not found")?;
    to_value(translation).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn del_translations(locale: &str) -> Result<(), JsValue> {
    let mut translations = TRANSLATIONS.lock().unwrap();
    translations.translations.remove(locale);
    Ok(())
}

#[wasm_bindgen]
pub fn has_locale(locale: &str) -> bool {
    let translations = TRANSLATIONS.lock().unwrap();
    translations.translations.contains_key(locale)
}

#[wasm_bindgen]
pub fn clear_all_translations() -> Result<(), JsValue> {
    let mut translations = TRANSLATIONS.lock().unwrap();
    translations.translations.clear();
    Ok(())
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
    let translations: HashMap<String, HashMap<String, TranslationValue>> = from_value(json)?;

    let mut locked_translations = TRANSLATIONS.lock().unwrap();
    for (locale, translation) in translations {
        locked_translations.translations.insert(locale, translation);
    }

    Ok(())
}

#[wasm_bindgen]
pub fn get_all_locales() -> Result<JsValue, JsValue> {
    let translations = TRANSLATIONS.lock().unwrap();
    let locales: Vec<String> = translations.translations.keys().cloned().collect();
    to_value(&locales).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn update_translation(locale: &str, key: &str, value: JsValue) -> Result<(), JsValue> {
    let mut translations = TRANSLATIONS.lock().unwrap();
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

#[wasm_bindgen]
pub fn format_translation(locale: &str, key: &str, args: JsValue) -> Result<String, JsValue> {
    // Блокируем глобальный словарь переводов
    let translations = TRANSLATIONS.lock().unwrap();

    // Получаем переводы для указанного локаля
    let translation_map = translations.translations.get(locale).ok_or_else(|| {
        JsValue::from_str("Locale not found")
    })?;

    // Ищем значение перевода по ключу
    let value = get_value_by_key(translation_map, key)?;

    // Убедимся, что значение - строка
    let value_str = if let TranslationValue::String(ref s) = value {
        s
    } else {
        return Err(JsValue::from_str("Translation is not a string"));
    };

    // Преобразуем входные аргументы в HashMap<String, String>
    let args_map: HashMap<String, String> = from_value(args).map_err(|e| {
        JsValue::from_str(&format!("Failed to parse arguments: {:?}", e))
    })?;

    // Форматируем строку с помощью метода format_string
    format_string(value_str, &args_map).map_err(|e| {
        JsValue::from_str(&format!("Error during formatting: {:?}", e))
    })
}

#[wasm_bindgen]
pub fn get_all_translations_for_locale(locale: &str) -> Result<JsValue, JsValue> {
    let translations = TRANSLATIONS.lock().unwrap();
    let translation = translations.translations.get(locale).ok_or("Locale not found")?;
    to_value(translation).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn get_all_translations() -> Result<JsValue, JsValue> {
    let translations = TRANSLATIONS.lock().unwrap();
    to_value(&translations.translations).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn has_key_in_translations(locale: &str, key: &str) -> bool {
    let translations = TRANSLATIONS.lock().unwrap();
    if let Some(translation_map) = translations.translations.get(locale) {
        get_value_by_key(translation_map, key).is_ok()
    } else {
        false
    }
}



