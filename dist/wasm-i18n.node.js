"use strict";require("./dist/wasm_i18n.js");let e=require("./dist/wasm_i18n.js");async function i(){await e.set_translations("en",JSON.stringify({welcome:"Hello {username}"})),await e.set_translations("en",JSON.stringify({test:{data:"1111"}}));const n=e.get_translations("en");console.log(n);const o=e.get_translation("en","welcome");console.log(o);const t=e.format_translation("en","welcome",{username:"Alice"});console.log(t),document.getElementById("welcome-message").innerText=t;const s=e.get_translation("en","test.data");console.log(s)}i();
