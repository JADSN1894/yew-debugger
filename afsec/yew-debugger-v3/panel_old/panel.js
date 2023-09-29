import init from './pkg/communication_grandchild_with_grandparent.js';

async function runDemo() {
    // Initialize the WASM module
    await init();

}

await runDemo()