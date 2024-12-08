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
    let _ = clear_all_translations();
    set_translations(
        "en",
        serde_wasm_bindgen::to_value(&json!({
            "welcome": "Welcome, {username}!"
        }))
        .unwrap(),
    )
    .unwrap();
    let translation = get_translation("en", "welcome").unwrap();
    let translation_str: String = serde_wasm_bindgen::from_value(translation).unwrap();

    assert_eq!(translation_str, "Welcome, {username}!");
}

#[wasm_bindgen_test]
fn test_has_translation() {
    let _ = clear_all_translations();
    set_translations(
        "en",
        serde_wasm_bindgen::to_value(&json!({
            "welcome": "Welcome, {username}!"
        }))
        .unwrap(),
    )
    .unwrap();
    assert!(has_translation("en", "welcome"));
    assert!(!has_translation("en", "missing_key"));
}

#[wasm_bindgen_test]
fn test_has_locale() {
    let _ = clear_all_translations();
    set_translations(
        "en",
        serde_wasm_bindgen::to_value(&json!({
            "welcome": "Welcome, {username}!"
        }))
        .unwrap(),
    )
    .unwrap();
    assert!(has_locale("en"));
    assert!(!has_locale("fr"));
}

#[wasm_bindgen_test]
fn test_format_translation() {
    let _ = clear_all_translations();
    set_translations(
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
    let formatted = format_translation("en", "welcome", args_js).unwrap();
    assert_eq!(formatted, "Welcome, Alice!");
}

#[wasm_bindgen_test]
fn test_get_all_locales() {
    let _ = clear_all_translations();
    set_translations(
        "en",
        serde_wasm_bindgen::to_value(&json!({
            "welcome": "Welcome, {username}!"
        }))
        .unwrap(),
    )
    .unwrap();
    set_translations(
        "fr",
        serde_wasm_bindgen::to_value(&json!({
            "welcome": "Bienvenue, {username}!"
        }))
        .unwrap(),
    )
    .unwrap();
    let locales: Vec<String> = from_value(get_all_locales().unwrap()).unwrap();
    assert_eq!(locales, vec!["en", "fr"]);
}

#[wasm_bindgen_test]
fn test_clear_all_translations() {
    let _ = clear_all_translations();
    set_translations(
        "en",
        serde_wasm_bindgen::to_value(&json!({
            "welcome": "Welcome, {username}!"
        }))
        .unwrap(),
    )
    .unwrap();
    clear_all_translations().unwrap();
    assert!(!has_locale("en"));
}

#[wasm_bindgen_test]
fn test_update_translation() {
    let _ = clear_all_translations();
    set_translations(
        "en",
        serde_wasm_bindgen::to_value(&json!({
            "welcome": "Welcome, {username}!"
        }))
        .unwrap(),
    )
    .unwrap();
    let new_value = JsValue::from_str("Hello, {username}!");
    update_translation("en", "welcome", new_value).unwrap();
    let translation = get_translation("en", "welcome").unwrap();

    // Преобразуйте JsValue обратно в строку для сравнения
    let translation_str: String = serde_wasm_bindgen::from_value(translation).unwrap();
    assert_eq!(translation_str, "Hello, {username}!");
}

#[wasm_bindgen_test]
fn test_get_all_translations() {
    let _ = clear_all_translations();
    set_translations(
        "en",
        serde_wasm_bindgen::to_value(&json!({
            "welcome": "Welcome, {username}!"
        }))
        .unwrap(),
    )
    .unwrap();
    let all_translations: HashMap<String, HashMap<String, String>> =
        from_value(get_all_translations().unwrap()).unwrap();
    assert!(all_translations.contains_key("en"));
}

#[wasm_bindgen_test]
fn test_performance() {
    use web_sys::window;

    // Получаем объект performance для измерения времени
    let performance = window()
        .and_then(|win| win.performance())
        .expect("Performance API is not available");

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
        set_translations("en", translations_js).unwrap();
    }
    let end = performance.now();
    let duration = end - start;
    console::log_1(&format!("Time to set 100000 translations: {} ms", duration).into());

    // Измеряем время на получение перевода
    let start = performance.now();
    for i in 0..100000 {
        let key = format!("key_{}", i);
        get_translation("en", &key).unwrap();
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
        format_translation("en", "key_500", args_js.clone()).unwrap();
    }
    let end = performance.now();
    let duration = end - start;
    console::log_1(&format!("Time to format 100000 translations: {} ms", duration).into());
}
