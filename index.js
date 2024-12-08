import { I18n } from "./pkg/wasm_i18n.js";

let i18n_instance = new I18n;

console.log(111, i18n_instance)

function mapToObject(map) {
    const obj = {};

    map.forEach((value, key) => {
        if (value instanceof Map) {
            // Если значение — это Map, рекурсивно преобразуем его
            obj[key] = mapToObject(value);
        } else {
            // Если значение не Map, просто присваиваем его в объект
            obj[key] = value;
        }
    });

    return obj;
}

async function run() {
    i18n_instance.setTranslations('en', {
        "welcome": "Hello {username}"
    });

    i18n_instance.setTranslations('en', {
        "test": {
            "data": '1111'
        }
    });

    const tr = i18n_instance.getTranslations('en')
    console.log('get_translations', tr, mapToObject(tr));

    const translation = i18n_instance.getTranslation('en', "welcome");
    console.log('get_translation welcome', translation);

    const formatted = i18n_instance.formatTranslation('en', 'welcome', { username: 'Alice' });
    console.log('formatted', formatted);

    document.getElementById('welcome-message').innerText = formatted;

    const test = i18n_instance.getTranslation('en', "test.data");
    console.log('get_translation test.data', test);

    const test1 = i18n_instance.getTranslation('en', "test");
    console.log('get_translation test', test1, mapToObject(test1));

    console.log('locales', i18n_instance.locales);
    console.log('translations', i18n_instance.translations);
}

run();

export { i18n_instance }