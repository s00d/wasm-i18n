use regex::Regex;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

pub fn format_string(template: &str, args: &HashMap<String, String>) -> Result<String, JsValue> {
    let re = Regex::new(r"\{(\w+)\}").unwrap();
    let result = re.replace_all(template, |caps: &regex::Captures| {
        let key = &caps[1];
        args.get(key).map_or(format!("{{{}}}", key), |value| value.to_string())
    });
    Ok(result.into_owned())
}