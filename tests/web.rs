//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_i18n::*;
use wasm_bindgen::JsValue;
use std::collections::HashMap;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_set_and_get_translation() {
    set_translations("en", r#"{"welcome": "Welcome, {username}!"}"#).unwrap();
    let translation = get_translation("en", "welcome").unwrap();
    assert_eq!(translation, r#""Welcome, {username}!""#);
}

#[wasm_bindgen_test]
fn test_has_translation() {
    set_translations("en", r#"{"welcome": "Welcome, {username}!"}"#).unwrap();
    assert!(has_translation("en", "welcome"));
    assert!(!has_translation("en", "missing_key"));
}

#[wasm_bindgen_test]
fn test_has_locale() {
    set_translations("en", r#"{"welcome": "Welcome, {username}!"}"#).unwrap();
    assert!(has_locale("en"));
    assert!(!has_locale("fr"));
}

#[wasm_bindgen_test]
fn test_format_translation() {
    set_translations("en", r#"{"welcome": "Welcome, {username}!"}"#).unwrap();
    let mut args = HashMap::new();
    args.insert("username".to_string(), "Alice".to_string());
    let args_js = JsValue::from_serde(&args).unwrap();
    let formatted = format_translation("en", "welcome", &args_js).unwrap();
    assert_eq!(formatted, "Welcome, Alice!");
}

#[wasm_bindgen_test]
fn test_get_all_locales() {
    set_translations("en", r#"{"welcome": "Welcome, {username}!"}"#).unwrap();
    set_translations("fr", r#"{"welcome": "Bienvenue, {username}!"}"#).unwrap();
    let locales: Vec<String> = get_all_locales().unwrap().into_serde().unwrap();
    assert_eq!(locales, vec!["en", "fr"]);
}

#[wasm_bindgen_test]
fn test_clear_all_translations() {
    set_translations("en", r#"{"welcome": "Welcome, {username}!"}"#).unwrap();
    clear_all_translations().unwrap();
    assert!(!has_locale("en"));
}


#[wasm_bindgen_test]
fn test_update_translation() {
    set_translations("en", r#"{"welcome": "Welcome, {username}!"}"#).unwrap();
    let new_value = JsValue::from_str("Hello, {username}!");
    update_translation("en", "welcome", new_value).unwrap();
    let translation = get_translation("en", "welcome").unwrap();
    assert_eq!(translation, r#""Hello, {username}!""#);
}

#[wasm_bindgen_test]
fn test_format_translation() {
    set_translations("en", r#"{"welcome": "Welcome, {username}!"}"#).unwrap();
    let mut args = HashMap::new();
    args.insert("username".to_string(), "Alice".to_string());
    let args_js = JsValue::from_serde(&args).unwrap();
    let formatted = format_translation("en", "welcome", &args_js).unwrap();
    assert_eq!(formatted, "Welcome, Alice!");
}

#[wasm_bindgen_test]
fn test_get_all_translations() {
    set_translations("en", r#"{"welcome": "Welcome, {username}!"}"#).unwrap();
    let all_translations: HashMap<String, HashMap<String, String>> =
        get_all_translations().unwrap().into_serde().unwrap();
    assert!(all_translations.contains_key("en"));
}

#[wasm_bindgen_test]
fn test_has_locale() {
    set_translations("en", r#"{"welcome": "Welcome, {username}!"}"#).unwrap();
    assert!(has_locale("en"));
    assert!(!has_locale("fr"));
}