use std::collections::HashMap;
use wasm_bindgen::JsValue;
use crate::TranslationValue;

pub fn get_value_by_key<'a>(
    map: &'a HashMap<String, TranslationValue>,
    key: &str,
) -> Result<&'a TranslationValue, String> {
    let keys: Vec<&str> = key.split('.').collect();
    let mut current = map;

    for k in &keys[..keys.len() - 1] {
        if let Some(TranslationValue::Nested(next_map)) = current.get(*k) {
            current = next_map;
        } else {
            return Err(format!("Key '{}' not found", k));
        }
    }

    current
        .get(&keys.last().unwrap().to_string())
        .ok_or_else(|| {
            format!(
                "Key '{}' not found in the provided translation map",
                keys.last().unwrap()
            )
        })
}

pub fn update_translation_recursive(
    current_map: &mut HashMap<String, TranslationValue>,
    keys: &[&str],
    value: TranslationValue,
) -> Result<(), JsValue> {
    if keys.len() == 1 {
        // Base case: if there's only one key left, insert the value
        current_map.insert(keys[0].to_string(), value);
        Ok(())
    } else {
        // Recursive case: traverse deeper into the nested structure
        let next_key = keys[0];
        let entry = current_map
            .entry(next_key.to_string())
            .or_insert_with(|| TranslationValue::Nested(HashMap::new()));

        // Check that the entry is not a string (i.e., it's a Nested variant)
        match entry {
            TranslationValue::String(_) => Err(
                JsValue::from("Invalid key path: found a string where a nested map was expected".to_string()),
            ),
            TranslationValue::Nested(ref mut nested_map) => {
                // Recursively call for the next level
                update_translation_recursive(nested_map, &keys[1..], value)
            }
        }
    }
}

pub fn remove_translation_recursive(
    current_map: &mut HashMap<String, TranslationValue>,
    keys: &[&str],
) -> Result<(), JsValue> {
    if keys.len() == 1 {
        // Base case: remove the key at the current level
        current_map.remove(keys[0]);
        Ok(())
    } else {
        // Recursive case: traverse deeper into the nested structure
        let next_key = keys[0];
        let entry = current_map
            .entry(next_key.to_string())
            .or_insert_with(|| TranslationValue::Nested(HashMap::new()));

        match entry {
            TranslationValue::String(_) => Err(
                JsValue::from("Invalid key path: found a string where a nested map was expected".to_string()),
            ),
            TranslationValue::Nested(ref mut nested_map) => {
                // Recursively call for the next level
                remove_translation_recursive(nested_map, &keys[1..])?;
                Ok(())
            }
        }
    }
}

pub fn format_string(
    template: &str,
    args: &HashMap<String, String>,
) -> Result<String, JsValue> {
    let mut result = template.to_string();
    for (key, value) in args {
        let placeholder = format!("{{{}}}", key);
        result = result.replace(&placeholder, value);
    }
    Ok(result)
}

pub fn merge_translation_values(
    existing: &mut TranslationValue,
    new: TranslationValue,
) {
    match (existing, new) {
        // Если оба значения - это Nested (вложенные объекты), объединяем их рекурсивно
        (
            TranslationValue::Nested(existing_map),
            TranslationValue::Nested(new_map),
        ) => {
            for (key, value) in new_map {
                existing_map
                    .entry(key)
                    .and_modify(|existing_value| merge_translation_values(existing_value, value.clone()))
                    .or_insert(value);
            }
        }
        // Во всех других случаях заменяем значение
        (existing_value, new_value) => {
            *existing_value = new_value; // Перезаписываем значение
        }
    }
}
