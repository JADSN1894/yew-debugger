import './output.css';
import init, { run_app } from './wasm/index.js';

async function main() {
    await init('./wasm/index_bg.wasm?url');
    run_app();
}

await main()