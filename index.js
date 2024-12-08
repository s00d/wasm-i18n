import * as wasmi18n from "./pkg/wasm_i18n.js";

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
    const version = wasmi18n.get_version();
    console.log('version', version);

    wasmi18n.set_translations('en', {
        "welcome": "Hello {username}"
    });

    wasmi18n.set_translations('en', {
        "test": {
            "data": '1111'
        }
    });

    const tr = wasmi18n.get_translations('en')
    const trObject = Object.fromEntries(tr);
    console.log('get_translations', tr, mapToObject(tr));

    const translation = wasmi18n.get_translation('en', "welcome");
    console.log('get_translation welcome', translation);

    const formatted = wasmi18n.format_translation('en', 'welcome', { username: 'Alice' });
    console.log('formatted', formatted);

    document.getElementById('welcome-message').innerText = formatted;

    const test = wasmi18n.get_translation('en', "test.data");
    console.log('get_translation test.data', test);

    const test1 = wasmi18n.get_translation('en', "test");
    console.log('get_translation test', test1, mapToObject(test1));
}

run();

export { wasmi18n }