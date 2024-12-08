Here is the updated README based on your request for using the `I18n` class for importing and setting translations:

---

# wasm-i18n

A Rust and WebAssembly project template for managing internationalization (i18n) translations in web applications using [wasm-pack](https://github.com/rustwasm/wasm-pack).

## About

This project provides a set of functions to manage internationalization (i18n) translations in web applications. It allows you to set, get, delete, and update translations for different locales, as well as load translations from a remote URL.

## 🚴 Usage

### Example Usage

```javascript
import { I18n } from 'wasm-i18n';

let i18n = new I18n();

async function run() {
    await i18n.setTranslations('en', {
        "welcome": "Hello {username}"
    });

    await i18n.setTranslations('en', {
        "test": {
            "data": '1111'
        }
    });

    const tr = i18n.getTranslations('en');
    console.log(tr);

    const translation = i18n.getTranslation('en', "welcome");
    console.log(translation);

    const formatted = i18n.formatTranslation('en', 'welcome', { username: 'Alice' });
    console.log(formatted);

    document.getElementById('welcome-message').innerText = formatted;

    const test = i18n.getTranslation('en', "test.data");
    console.log(test);
}

run();
```

## API Documentation

### `setTranslations(locale: string, obj: any): void`

Sets translations for a specific locale. If translations already exist for the locale, they will be merged with the new translations.

```javascript
i18n.setTranslations('en', {
    "hello": "Hello",
    "world": "World"
});
```

### `getTranslations(locale: string): any`

Gets all translations for a specific locale.

```javascript
const translations = i18n.getTranslations('en');
console.log(translations);
```

### `delTranslations(locale: string): void`

Deletes all translations for a specific locale.

```javascript
i18n.delTranslations('en');
```

### `delTranslation(locale: string, key: string): void`

Deletes a specific translation key for a locale.

```javascript
i18n.delTranslation('en', 'hello');
```

### `getTranslation(locale: string, key: string): any`

Gets the translation for a specific key in a locale.

```javascript
const translation = i18n.getTranslation('en', 'hello');
console.log(translation);
```

### `hasTranslation(locale: string, key: string): boolean`

Checks if a specific translation key exists in a locale.

```javascript
const exists = i18n.hasTranslation('en', 'hello');
console.log(exists);
```

### `hasLocale(locale: string): boolean`

Checks if a specific locale exists.

```javascript
const exists = i18n.hasLocale('en');
console.log(exists);
```

### `formatTranslation(locale: string, key: string, args: any): string`

Formats a translation string with provided arguments.

```javascript
const formatted = i18n.formatTranslation('en', 'welcome', { username: 'Alice' });
console.log(formatted);
```

### `loadTranslations(url: string): Promise<void>`

Loads translations from a remote URL.

```javascript
await i18n.loadTranslations('https://example.com/translations.json');
```

### `getAllLocales(): Promise<Array<string>>`

Gets all available locales.

```javascript
const locales = await i18n.getAllLocales();
console.log(locales);
```

### `getAllTranslationsForLocale(locale: string): Promise<any>`

Gets all translations for a specific locale.

```javascript
const translations = await i18n.getAllTranslationsForLocale('en');
console.log(translations);
```

### `clearAllTranslations(): void`

Clears all translations.

```javascript
i18n.clearAllTranslations();
```

### `updateTranslation(locale: string, key: string, value: any): void`

Updates a specific translation key for a locale.

```javascript
i18n.updateTranslation('en', 'hello', 'Hello, World!');
```

### `getAllTranslations(): any`

Gets all translations for all locales.

```javascript
const all_translations = i18n.getAllTranslations();
console.log(all_translations);
```

### `hasKeyInTranslations(locale: string, key: string): boolean`

Checks if a specific key exists in the translations for a locale.

```javascript
const exists = i18n.hasKeyInTranslations('en', 'hello');
console.log(exists);
```

### Getter Methods

| Method         | Description                                         | Example                                                                                                           |
|----------------|-----------------------------------------------------|-------------------------------------------------------------------------------------------------------------------|
| `version`      | Returns the version of the project at compile-time. | ```js let version = i18n.version; console.log(version); // "1.0.0" ```                                            |
| `locales`      | Retrieves all available locales.                    | ```js let locales = i18n.locales; console.log(locales); // ["en", "fr", "de", ...] ```                            |
| `translations` | Retrieves all translations for all locales.         | ```js let translations = i18n.translations; console.log(translations); // { "en": { "hello": "Hello" }, ... } ``` |


## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

---

Let me know if you need further modifications!