use std::collections::HashMap;
use wasm_bindgen::JsValue;

pub fn format_string(template: &str, args: &HashMap<String, String>) -> Result<String, JsValue> {
    let mut result = template.to_string();
    for (key, value) in args {
        let placeholder = format!("{{{}}}", key);
        result = result.replace(&placeholder, value);
    }
    Ok(result)
}
