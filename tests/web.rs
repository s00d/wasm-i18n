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