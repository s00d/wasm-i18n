//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use serde_json::json;
use serde_wasm_bindgen::from_value;
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use wasm_i18n::*;
use web_sys::console;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_set_and_get_translation() {
    let i18n = I18n::new();
    let _ = i18n.clear_all_translations();
    i18n.set_translations(
        "en",
        serde_wasm_bindgen::to_value(&json!({
            "welcome": "Welcome, {username}!"
        }))
        .unwrap(),
    )
    .unwrap();
    let translation = i18n.get_translation("en", "welcome").unwrap();
    let translation_str: String = serde_wasm_bindgen::from_value(translation).unwrap();

    assert_eq!(translation_str, "Welcome, {username}!");
}

#[wasm_bindgen_test]
fn test_has_translation() {
    let i18n = I18n::new();
    let _ = i18n.clear_all_translations();
    i18n.set_translations(
        "en",
        serde_wasm_bindgen::to_value(&json!({
            "welcome": "Welcome, {username}!"
        }))
        .unwrap(),
    )
    .unwrap();
    assert!(i18n.has_translation("en", "welcome"));
    assert!(!i18n.has_translation("en", "missing_key"));
}

#[wasm_bindgen_test]
fn test_has_locale() {
    let i18n = I18n::new();
    let _ = i18n.clear_all_translations();
    i18n.set_translations(
        "en",
        serde_wasm_bindgen::to_value(&json!({
            "welcome": "Welcome, {username}!"
        }))
        .unwrap(),
    )
    .unwrap();
    assert!(i18n.has_locale("en"));
    assert!(!i18n.has_locale("fr"));
}

#[wasm_bindgen_test]
fn test_format_translation() {
    let i18n = I18n::new();
    let _ = i18n.clear_all_translations();
    i18n.set_translations(
        "en",
        serde_wasm_bindgen::to_value(&json!({
            "welcome": "Welcome, {username}!"
        }))
        .unwrap(),
    )
    .unwrap();
    let mut args = HashMap::new();
    args.insert("username".to_string(), "Alice".to_string());
    let args_js = serde_wasm_bindgen::to_value(&args).unwrap();
    let formatted = i18n.format_translation("en", "welcome", args_js).unwrap();
    assert_eq!(formatted, "Welcome, Alice!");
}

#[wasm_bindgen_test]
fn test_get_all_locales() {
    let i18n = I18n::new();
    let _ = i18n.clear_all_translations();
    i18n.set_translations(
        "en",
        serde_wasm_bindgen::to_value(&json!({
            "welcome": "Welcome, {username}!"
        }))
        .unwrap(),
    )
    .unwrap();
    i18n.set_translations(
        "fr",
        serde_wasm_bindgen::to_value(&json!({
            "welcome": "Bienvenue, {username}!"
        }))
        .unwrap(),
    )
    .unwrap();
    let locales: Vec<String> = from_value(i18n.locales().unwrap()).unwrap();
    assert_eq!(locales, vec!["en", "fr"]);
}

#[wasm_bindgen_test]
fn test_clear_all_translations() {
    let i18n = I18n::new();
    let _ = i18n.clear_all_translations();
    i18n.set_translations(
        "en",
        serde_wasm_bindgen::to_value(&json!({
            "welcome": "Welcome, {username}!"
        }))
        .unwrap(),
    )
    .unwrap();
    i18n.clear_all_translations().unwrap();
    assert!(!i18n.has_locale("en"));
}

#[wasm_bindgen_test]
fn test_update_translation() {
    let i18n = I18n::new();
    let _ = i18n.clear_all_translations();
    i18n.set_translations(
        "en",
        serde_wasm_bindgen::to_value(&json!({
            "welcome": "Welcome, {username}!"
        }))
        .unwrap(),
    )
    .unwrap();
    let new_value = JsValue::from_str("Hello, {username}!");
    i18n.update_translation("en", "welcome", new_value).unwrap();
    let translation = i18n.get_translation("en", "welcome").unwrap();

    // Преобразуйте JsValue обратно в строку для сравнения
    let translation_str: String = serde_wasm_bindgen::from_value(translation).unwrap();
    assert_eq!(translation_str, "Hello, {username}!");
}

#[wasm_bindgen_test]
fn test_get_all_translations() {
    let i18n = I18n::new();
    let _ = i18n.clear_all_translations();
    i18n.set_translations(
        "en",
        serde_wasm_bindgen::to_value(&json!({
            "welcome": "Welcome, {username}!"
        }))
        .unwrap(),
    )
    .unwrap();
    let all_translations: HashMap<String, HashMap<String, String>> =
        serde_wasm_bindgen::from_value(i18n.translations().unwrap()).unwrap();

    assert!(all_translations.contains_key("en"));
    assert_eq!(all_translations["en"]["welcome"], "Welcome, {username}!");
}

#[wasm_bindgen_test]
fn test_performance() {
    use web_sys::window;

    // Получаем объект performance для измерения времени
    let performance = window()
        .and_then(|win| win.performance())
        .expect("Performance API is not available");

    let i18n = I18n::new();

    // Измеряем время на установку переводов
    let start = performance.now();
    for i in 0..100000 {
        let key = format!("key_{}", i);
        let translation = format!("Translation {}", i);
        let translations = format!(r#"{{"{}": "{}"}}"#, key, translation);
        let translations_js = serde_wasm_bindgen::to_value(
            &serde_json::from_str::<serde_json::Value>(&translations).unwrap(),
        )
        .unwrap();
        i18n.set_translations("en", translations_js).unwrap();
    }
    let end = performance.now();
    let duration = end - start;
    console::log_1(&format!("Time to set 100000 translations: {} ms", duration).into());

    // Измеряем время на получение перевода
    let start = performance.now();
    for i in 0..100000 {
        let key = format!("key_{}", i);
        i18n.get_translation("en", &key).unwrap();
    }
    let end = performance.now();
    let duration = end - start;
    console::log_1(&format!("Time to get 100000 translations: {} ms", duration).into());

    // Измеряем время на форматирование перевода
    let mut args = HashMap::new();
    args.insert("username".to_string(), "Alice".to_string());
    let args_js = serde_wasm_bindgen::to_value(&args).unwrap();
    let start = performance.now();
    for _ in 0..100000 {
        i18n.format_translation("en", "key_500", args_js.clone()).unwrap();
    }
    let end = performance.now();
    let duration = end - start;
    console::log_1(&format!("Time to format 100000 translations: {} ms", duration).into());
}
