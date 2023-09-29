import './app.css';

// document.querySelector('#app').innerHTML = `
//   <button class="btn">Hello daisyUI</button>
// `;

import init from './wasm/communication_grandchild_with_grandparent.js';

await init();