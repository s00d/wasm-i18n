import * as wasmi18n from "./dist/wasm_i18n.js";

async function run() {
    wasmi18n.set_translations('en', JSON.stringify({
        "welcome": "Hello {username}"
    }));

    wasmi18n.set_translations('en', JSON.stringify({
        "test": {
            "data": '1111'
        }
    }));

    const tr = wasmi18n.get_translations('en')

    console.log(tr);

    const translation = wasmi18n.get_translation('en', "welcome");
    console.log(translation);

    const formatted = wasmi18n.format_translation('en', 'welcome', { username: 'Alice' });
    console.log(formatted);

    document.getElementById('welcome-message').innerText = formatted;

    const test = wasmi18n.get_translation('en', "test.data");
    console.log(test);

    const test1 = wasmi18n.get_translation('en', "test");
    console.log(test1);
}

run();

export { wasmi18n }