# wasm-i18n

A Rust and WebAssembly project template for managing internationalization (i18n) translations in web applications using [wasm-pack](https://github.com/rustwasm/wasm-pack).

## About

This project provides a set of functions to manage internationalization (i18n) translations in web applications. It allows you to set, get, delete, and update translations for different locales, as well as load translations from a remote URL.

## 🚴 Usage

### Example Usage

```javascript
import initSync, { set_translations, get_translation, get_translations, format_translation } from 'wasm-i18n';

async function run() {
    await initSync();

    await set_translations('en', JSON.stringify({
        "welcome": "Hello {username}"
    }));

    await set_translations('en', JSON.stringify({
        "test": {
            "data": '1111'
        }
    }));

    const tr = get_translations('en');
    console.log(tr);

    const translation = get_translation('en', "welcome");
    console.log(translation);

    const formatted = format_translation('en', 'welcome', { username: 'Alice' });
    console.log(formatted);

    document.getElementById('welcome-message').innerText = formatted;

    const test = get_translation('en', "test.data");
    console.log(test);
}

run();
```

## API Documentation

### `set_translations(locale: string, json: string): Promise<void>`

Sets translations for a specific locale. If translations already exist for the locale, they will be merged with the new translations.

```javascript
await set_translations('en', JSON.stringify({
    "hello": "Hello",
    "world": "World"
}));
```

### `set_translations_from_object(locale: string, obj: object): Promise<void>`

Sets translations for a specific locale using a JavaScript object.

```javascript
await set_translations_from_object('en', {
    "hello": "Hello",
    "world": "World"
});
```

### `get_translations(locale: string): Promise<string>`

Gets all translations for a specific locale as a JSON string.

```javascript
const translations = await get_translations('en');
console.log(translations);
```

### `del_translations(locale: string): Promise<void>`

Deletes all translations for a specific locale.

```javascript
await del_translations('en');
```

### `del_translation(locale: string, key: string): Promise<void>`

Deletes a specific translation key for a locale.

```javascript
await del_translation('en', 'hello');
```

### `get_translation(locale: string, key: string): Promise<string>`

Gets the translation for a specific key in a locale.

```javascript
const translation = await get_translation('en', 'hello');
console.log(translation);
```

### `has_translation(locale: string, key: string): boolean`

Checks if a specific translation key exists in a locale.

```javascript
const exists = has_translation('en', 'hello');
console.log(exists);
```

### `has_locale(locale: string): boolean`

Checks if a specific locale exists.

```javascript
const exists = has_locale('en');
console.log(exists);
```

### `format_translation(locale: string, key: string, args: object): Promise<string>`

Formats a translation string with provided arguments.

```javascript
const formatted = await format_translation('en', 'welcome', { username: 'Alice' });
console.log(formatted);
```

### `load_translations(url: string): Promise<void>`

Loads translations from a remote URL.

```javascript
await load_translations('https://example.com/translations.json');
```

### `get_all_locales(): Promise<Array<string>>`

Gets all available locales.

```javascript
const locales = await get_all_locales();
console.log(locales);
```

### `get_all_translations_for_locale(locale: string): Promise<object>`

Gets all translations for a specific locale.

```javascript
const translations = await get_all_translations_for_locale('en');
console.log(translations);
```

### `clear_all_translations(): Promise<void>`

Clears all translations.

```javascript
await clear_all_translations();
```

### `update_translation(locale: string, key: string, value: string): Promise<void>`

Updates a specific translation key for a locale.

```javascript
await update_translation('en', 'hello', 'Hello, World!');
```

### `get_all_translations(): Promise<object>`

Gets all translations for all locales.

```javascript
const all_translations = await get_all_translations();
console.log(all_translations);
```

### `has_key_in_translations(locale: string, key: string): boolean`

Checks if a specific key exists in the translations for a locale.

```javascript
const exists = has_key_in_translations('en', 'hello');
console.log(exists);
```

### `add_locale(locale: string): Promise<void>`

Adds a new locale with empty translations.

```javascript
await add_locale('fr');
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.