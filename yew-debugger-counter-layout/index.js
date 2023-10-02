import './output.css';
import init, { run_app } from './wasm/index.js';

await init().then(() => run_app());