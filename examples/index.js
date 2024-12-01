import { set_translations, get_translation, get_translations, format_translation } from '../pkg/bundler';

async function run() {
    await set_translations('en', JSON.stringify({
        "welcome": "Hello {username}"
    }));

    await set_translations('en', JSON.stringify({
        "test": {
            "data": '1111'
        }
    }));

    const tr = get_translations('en')

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